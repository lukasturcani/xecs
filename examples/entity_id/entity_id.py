import xecs as xx


class One(xx.Component):
    pass


class Two(xx.Component):
    pass


def system(query: xx.Query[tuple[xx.EntityId, One, Two]]) -> None:
    entity_id, _, _ = query.result()
    print("Entities with both components are:")
    print(entity_id)


def main() -> None:
    app = xx.RealTimeApp(num_entities=30)
    app.add_pool(One.create_pool(20))
    app.add_pool(Two.create_pool(10))
    app.add_startup_system(spawn_entities)
    app.add_system(system)
    app.update()


def spawn_entities(commands: xx.Commands) -> None:
    commands.spawn((One,), 5)
    commands.spawn((Two,), 5)
    commands.spawn((One, Two), 5)


if __name__ == "__main__":
    main()
