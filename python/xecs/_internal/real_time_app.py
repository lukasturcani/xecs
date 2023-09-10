import inspect
import typing
from collections import abc
from time import sleep
from typing import Any

from xecs._internal.commands import Commands
from xecs._internal.component import (
    Component,
    ComponentPool,
    ComponentT,
)
from xecs._internal.query import Query
from xecs._internal.resource import Resource
from xecs._internal.systems import (
    FixedTimeStepSystems,
    FixedTimeStepSystemSpec,
    NonQueryParameter,
    PendingStartupSystems,
    PendingSystems,
    StartupSystems,
    System,
    SystemParameter,
    Systems,
    SystemSignatureError,
    SystemSpec,
)
from xecs._internal.time import Time
from xecs._internal.world import World
from xecs.xecs import Duration, Instant, RustApp

P = typing.ParamSpec("P")
R = typing.TypeVar("R")


class RealTimeAppPlugin:
    """
    A base class for plugins for :class:`.RealTimeApp`.
    """

    def build(self, app: "RealTimeApp") -> None:
        """
        Add the plugin to an `app`.

        Parameters:
            app: The app to add the plugin to.
        """
        pass


class RealTimeApp:
    """
    An app which runs in real time.
    """

    def __init__(self) -> None:
        self.world = World()
        self.add_resource(PendingStartupSystems([]))
        self.add_resource(StartupSystems([]))
        self.add_resource(PendingSystems([]))
        self.add_resource(Systems([]))
        self.add_resource(FixedTimeStepSystems([]))

        self._rust_app = RustApp(
            num_pools=len(Component.component_ids),
            num_queries=Query.p_num_queries,
        )
        self._commands = Commands.p_new(self._rust_app, self.world)
        self._has_run_startup_systems = False

    def add_plugin(self, plugin: RealTimeAppPlugin) -> None:
        """
        Add a plugin.

        Parameters:
            plugin: The plugin.
        """
        plugin.build(self)

    def add_resource(self, resource: Resource) -> None:
        """
        Add a resource.

        Parameters:
            resource: The resource.
        """
        self.world.add_resource(resource)

    def add_startup_system(self, system: System) -> None:
        """
        Add a startup system.

        Parameters:
            system: The startup system.
        """
        self.world.get_resource(PendingStartupSystems).systems.append(system)

    def add_system(
        self,
        system: System,
        run_condition: Duration | None = None,
    ) -> None:
        """
        Add a system.

        Parameters:
            system:
                The system.
            run_condition:
                The time step between runs of the system.
                If ``None`` the system will run every frame.
        """
        self.world.get_resource(PendingSystems).systems.append(
            (system, run_condition)
        )

    def _get_system_args(
        self,
        system: abc.Callable[P, R],
    ) -> tuple[dict[str, Query[Any]], dict[str, NonQueryParameter]]:
        query_args: dict[str, Query[Any]] = {}
        other_args: dict[str, NonQueryParameter] = {}
        for name, parameter in inspect.signature(system).parameters.items():
            if typing.get_origin(parameter.annotation) is Query:
                (component_tuple,) = typing.get_args(parameter.annotation)
                if issubclass(component_tuple, Component):
                    query_id = self._rust_app.add_query(
                        first_component=Component.component_ids[
                            component_tuple
                        ],
                        other_components=[],
                    )
                    query_args[name] = Query.p_new(
                        query_id, [component_tuple], False
                    )

                else:
                    components = typing.get_args(component_tuple)
                    component_ids = [
                        Component.component_ids[component]
                        for component in components
                    ]

                    query_id = self._rust_app.add_query(
                        first_component=component_ids[0],
                        other_components=component_ids[1:],
                    )
                    query_args[name] = Query.p_new(query_id, components, True)

            elif parameter.annotation is Commands:
                other_args[name] = self._commands
            elif parameter.annotation is World:
                other_args[name] = self.world
            elif issubclass(parameter.annotation, Resource):
                other_args[name] = self.world.get_resource(
                    parameter.annotation
                )
            else:
                expected_type = " | ".join(
                    arg.__name__ for arg in typing.get_args(SystemParameter)
                )
                raise SystemSignatureError(
                    f'annotation of parameter "{name}" in '
                    f'"{system.__name__}" is "{parameter.annotation}" '
                    "but needs to be "
                    f"{expected_type}"
                )
        return query_args, other_args

    def _run_query(self, query: Query[Any]) -> None:
        component_indices = self._rust_app.run_query(query.p_query_id)
        query.p_result = tuple(
            pool.p_component.p_new_view_with_indices(indices)
            for pool, indices in zip(
                (
                    self.world.p_get_pool(component)
                    for component in query.p_components
                ),
                iter(lambda: component_indices.next(), None),
                strict=True,
            )
        )
        if not query.p_tuple_query:
            query.p_result = query.p_result[0]

    def _process_pending_startup_systems(self) -> None:
        pending_startup_systems = self.world.get_resource(
            PendingStartupSystems
        )
        startup_systems = self.world.get_resource(StartupSystems)
        for system in pending_startup_systems.systems:
            query_args, other_args = self._get_system_args(system)
            startup_systems.systems.append(
                SystemSpec(
                    function=system,
                    query_args=query_args,
                    other_args=other_args,
                )
            )
        pending_startup_systems.systems = []

    def _process_pending_systems(self) -> None:
        pending_systems = self.world.get_resource(PendingSystems)
        systems = self.world.get_resource(Systems)
        fixed_time_step_systems = self.world.get_resource(FixedTimeStepSystems)
        for system, run_condition in pending_systems.systems:
            query_args, other_args = self._get_system_args(system)

            match run_condition:
                case Duration():
                    fixed_time_step_systems.systems.append(
                        FixedTimeStepSystemSpec(
                            system, query_args, other_args, run_condition
                        )
                    )
                case None:
                    systems.systems.append(
                        SystemSpec(system, query_args, other_args)
                    )
        pending_systems.systems = []

    def _run_startup_systems(self) -> None:
        self._has_run_startup_systems = True
        for system in self.world.get_resource(StartupSystems).systems:
            for query in system.query_args.values():
                self._run_query(query)

            system.function(
                **system.query_args,
                **system.other_args,
            )

    def _run_systems(self) -> None:
        for system in self.world.get_resource(Systems).systems:
            for query in system.query_args.values():
                self._run_query(query)

            system.function(
                **system.query_args,
                **system.other_args,
            )

    def _run_fixed_time_step_systems(
        self,
        time_since_last_update: Duration,
    ) -> None:
        for system in self.world.get_resource(FixedTimeStepSystems).systems:
            system.time_to_simulate += time_since_last_update
            while system.time_to_simulate >= system.time_step:
                for query in system.query_args.values():
                    self._run_query(query)

                system.function(
                    **system.query_args,
                    **system.other_args,
                )
                system.time_to_simulate -= system.time_step

    def update(self) -> None:
        """
        Run the app for a single step.
        """
        if not self.world.has_resource(Time):
            self.world.add_resource(Time.default())
        if not self._has_run_startup_systems:
            self._process_pending_startup_systems()
            self._run_startup_systems()
        self._process_pending_systems()
        self._update()

    def _update(self) -> None:
        time = self.world.get_resource(Time)
        time.update()
        self._run_systems()
        self._run_fixed_time_step_systems(time.delta())

    def run(
        self,
        frame_time: Duration = Duration.from_nanos(int(1e9 / 60)),
        max_run_time: Duration | None = None,
    ) -> None:
        """
        Run the app continuously.

        Parameters:
            frame_time: The time between frames.
            max_run_time: The maximum time to run the app for.
        """
        if not self.world.has_resource(Time):
            self.world.add_resource(Time.default())
        if not self._has_run_startup_systems:
            self._process_pending_startup_systems()
            self._run_startup_systems()
        self._process_pending_systems()
        self._run(frame_time, max_run_time)

    def _run(
        self,
        frame_time: Duration,
        max_run_time: Duration | None,
    ) -> None:
        time = self.world.get_resource(Time)
        while True:
            start = Instant.now()
            self._update()
            if max_run_time is not None and time.elapsed() >= max_run_time:
                break
            sleep_time = frame_time.saturating_sub(start.elapsed())
            sleep(sleep_time.as_nanos() / 1e9)

    def add_pool(self, pool: ComponentPool[ComponentT]) -> None:
        """
        Add a preallocated pool of components.

        The `pool` will be used to hold any components which
        are spawned during runtime.

        Parameters:
            pool: The component pool.
        """
        component_id = Component.component_ids[type(pool.p_component)]
        self._rust_app.add_pool(component_id, pool.p_capacity)
        self.world.add_pool(pool)
