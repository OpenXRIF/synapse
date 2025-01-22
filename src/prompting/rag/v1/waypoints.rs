pub fn get_waypoints_list(_: &str) -> String {
    // TODO: Retrieve waypoints from cache
    r#"
    Waypoint(
        name="E7 1st floor Elevators",
        floor=1,
        keywords=["elevator"],
    ),
    Waypoint(
        name="E7 North Entrance",
        floor=1,
        keywords=["entrance"],
    ),
    Waypoint(
        name="E7 East Entrance",
        floor=1,
        keywords=["entrance"],
    ),
    Waypoint(
        name="E7 South Entrance",
        floor=1,
        keywords=["entrance"],
    ),
    Waypoint(
        name="E7 Coffee and Donuts",
        floor=1,
        keywords=["food and drink", "coffee", "breakfast", "snacks"],
        aliases=["CnD"]
    ),
    Waypoint(
        name="Outreach Classroom",
        floor=1,
        keywords=["classroom", "lecture hall"],
    ),
    Waypoint(
        name="RoboHub Entrance",
        floor=1,
        keywords=["robots"],
    ),
    Waypoint(
        name="Vending Machine",
        floor=1,
        keywords=["food and drink", "snacks"],
    ),
    Waypoint(
        name="Room 2241",
        floor=2,
        keywords=["office", "Zach"],
    )
    "#
    .to_string()
}
