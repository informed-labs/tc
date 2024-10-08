use crate::resolver::Event;
use aws::eventbridge;
use aws::Env;
use colored::Colorize;

async fn make_event(env: &Env, event: Event) -> eventbridge::Event {
    let Event {
        name,
        rule_name,
        bus,
        target,
        pattern,
        ..
    } = event;

    let client = eventbridge::make_client(&env).await;
    let appsync = eventbridge::make_appsync_params(&target.name);
    let input_transformer = match target.input_template.clone() {
        Some(_) => Some(eventbridge::make_input_transformer(
            target.input_paths_map,
            target.input_template,
        )),
        None => None,
    };
    let aws_target = eventbridge::make_target(
        &target.id,
        &target.arn,
        &target.role_arn,
        &target.kind,
        input_transformer,
        Some(appsync),
    );
    eventbridge::Event {
        client: client,
        name: name,
        rule_name: rule_name,
        bus: bus,
        role: String::from(&target.role_arn),
        target: aws_target,
        pattern: serde_json::to_string(&pattern).unwrap(),
    }
}

pub async fn create_event(env: &Env, event: Event) {
    println!("Creating event {}", &event.name.green());
    let target_event = make_event(env, event.clone()).await;

    let target_arn = &event.target.arn;
    if !target_arn.is_empty() {
        let rule_arn = target_event.clone().create_rule().await;

        if !rule_arn.is_empty() {
            target_event.clone().put_target().await;
        }
    }
}

pub async fn create(env: &Env, events: Vec<Event>) {
    for event in events {
        create_event(env, event).await;
    }
}

pub async fn delete_event(env: &Env, event: Event) {
    println!("Deleting event {}", &event.name.red());

    let target_event = make_event(env, event.clone()).await;
    target_event.clone().remove_targets(&event.target.id).await;
    target_event.clone().delete_rule().await;
}

pub async fn delete(env: &Env, events: Vec<Event>) {
    for event in events {
        delete_event(env, event).await;
    }
}
