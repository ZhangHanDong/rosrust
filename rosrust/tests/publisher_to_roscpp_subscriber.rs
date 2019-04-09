use crossbeam::channel::unbounded;
use rosrust;
use std::process::Command;

mod util;

mod msg {
    rosrust::rosmsg_include!(std_msgs / String, rosgraph_msgs / Log);
}

#[test]
fn publisher_to_roscpp_subscriber() {
    let _roscore = util::run_roscore_for(util::Language::Cpp, util::Feature::Publisher);
    let _subscriber = util::ChildProcessTerminator::spawn(
        Command::new("rosrun")
            .arg("roscpp_tutorials")
            .arg("listener"),
    );

    rosrust::init("hello_world_talker");

    let (tx, rx) = unbounded();

    let _log_subscriber =
        rosrust::subscribe::<msg::rosgraph_msgs::Log, _>("/rosout_agg", 100, move |data| {
            tx.send((data.level, data.msg)).unwrap();
        })
        .unwrap();

    let publisher = rosrust::publish::<msg::std_msgs::String>("chatter", 100).unwrap();

    let mut message = msg::std_msgs::String::default();
    message.data = "hello world".to_owned();

    util::test_publisher(&publisher, &message, &rx, r"^I heard: \[hello world\]$", 50);

    assert_eq!(publisher.subscriber_count(), 1);
}
