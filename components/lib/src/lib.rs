mod builder;
mod compiler;
mod deployer;
mod emulator;
mod invoker;
mod lister;
mod publisher;
mod resolver;
mod scaffolder;
mod tester;

use aws::Env;
use colored::Colorize;
use compiler::Topology;
use kit as u;
use resolver::Plan;
use std::panic;
use std::time::Instant;

pub fn init_env(profile: Option<String>) -> Env {
    let profile = u::maybe_string(profile, "dev");
    Env { name: profile }
}

pub struct BuildOpts {
    pub pack: bool,
    pub no_docker: bool,
    pub trace: bool,
    pub clean: bool,
    pub delete: bool,
    pub parallel: bool,
    pub merge: bool,
    pub dirty: bool,
}

pub async fn build(kind: Option<String>, name: Option<String>, dir: &str, opts: BuildOpts) {
    let kind = builder::determine_kind(kind);
    let name = u::maybe_string(name, u::basedir(&u::pwd()));

    let BuildOpts {
        pack,
        no_docker,
        trace,
        clean,
        dirty,
        merge,
        ..
    } = opts;

    if pack {
        builder::pack_all(dir);
    } else if clean {
        builder::clean(dir);
    } else if merge {
        let layers = compiler::find_layers();
        let mergeable_layers = builder::mergeable_layers(layers);
        builder::merge(&name, mergeable_layers);
    } else {
        builder::build(&dir, &name, &kind, no_docker, trace, dirty).await;
    }
}

pub struct PublishOpts {
    pub trace: bool,
    pub promote: bool,
    pub demote: bool,
    pub version: Option<String>,
}

pub async fn publish(
    profile: Option<String>,
    kind: Option<String>,
    name: Option<String>,
    dir: &str,
    opts: PublishOpts,
) {
    let env = init_env(profile);
    let PublishOpts {
        promote,
        demote,
        version,
        ..
    } = opts;

    if promote {
        let lang = &compiler::guess_lang(&dir);
        let bname = u::maybe_string(name, u::basedir(&u::pwd()));
        publisher::promote(&env, &bname, &lang, version).await;
    } else if demote {
        let lang = "python3.10";
        publisher::demote(&env, name, &lang).await;
    } else {
        let lang = &compiler::guess_lang(&dir);
        let target = compiler::determine_target(dir);
        let name = u::maybe_string(name, u::basedir(dir));
        let kind = u::maybe_string(kind, "deps");
        let builds = builder::just_build_out(&dir, &name, &lang, &target);
        match kind.as_ref() {
            "deps" | "extension" => {
                for build in builds {
                    publisher::publish_deps(
                        &env,
                        &build.dir,
                        &build.zipfile,
                        &build.lang,
                        &build.name,
                        &build.target,
                    )
                    .await
                }
            }
            _ => (),
        }
    }
}

pub async fn list_published_assets(profile: Option<String>) {
    let env = init_env(profile);
    publisher::list(&env).await
}

pub async fn test() {
    let dir = u::pwd();
    let spec = compiler::compile(&dir, false);
    for (path, function) in spec.functions {
        tester::test(&path, function).await;
    }
}

pub struct CompileOpts {
    pub versions: bool,
    pub recursive: bool,
    pub component: Option<String>,
    pub format: Option<String>,
}

pub async fn compile(opts: CompileOpts) -> String {
    let CompileOpts {
        recursive,
        component,
        format,
        ..
    } = opts;

    let dir = u::pwd();
    let format = u::maybe_string(format, "json");

    match component {
        Some(c) => compiler::show_component(&c, &format),
        None => {
            let topology = compiler::compile(&dir, recursive);
            u::pretty_json(topology)
        }
    }
}

pub async fn resolve(
    profile: Option<String>,
    sandbox: Option<String>,
    component: Option<String>,
    recursive: bool,
) -> String {
    let resolve = resolver::should_resolve(component.clone());
    let env = Env::maybe(profile);
    let topology = compiler::compile(&u::pwd(), recursive);
    let plans = resolver::resolve(&env, sandbox, &topology, resolve).await;
    let component = u::maybe_string(component, "all");
    resolver::render(plans, &component)
}

async fn create_plan(plan: Plan, _notify: bool) {
    let Plan { functions, .. } = plan.clone();

    for (_, function) in functions {
        let lang = function.runtime.lang;
        let mtask = &function.tasks.get("build");
        let dir = &function.dir.unwrap();
        match mtask {
            Some(task) => {
                builder::pack(&lang, dir, &task);
            }
            _ => {
                builder::pack(&lang, dir, "zip -q lambda.zip *.rb *.py");
            }
        }
    }
    deployer::create(plan).await;
}

fn count_of(topology: &Topology) -> String {
    let Topology { functions, .. } = topology;
    format!("{} functions", functions.len())
}

pub async fn create(
    profile: Option<String>,
    sandbox: Option<String>,
    notify: bool,
    recursive: bool,
) {
    let dir = u::pwd();
    let start = Instant::now();

    println!("Compiling topology");
    let topology = compiler::compile(&dir, recursive);

    println!("Resolving topology ({}) ", count_of(&topology).cyan());
    let env = Env::maybe(profile);
    let plans = resolver::resolve(&env, sandbox, &topology, true).await;
    for plan in plans.clone() {
        create_plan(plan, notify).await;
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:#}", u::time_format(duration));
}

async fn update_plan(plan: Plan) {
    let Plan { dir, .. } = plan.clone();
    builder::pack_all(&dir);
    deployer::update(plan.clone()).await;
}

pub async fn update(profile: Option<String>, sandbox: Option<String>, recursive: bool) {
    let start = Instant::now();

    let env = Env::maybe(profile);
    println!("Compiling topology");
    let topology = compiler::compile(&u::pwd(), recursive);

    println!("Resolving topology ({}) ", count_of(&topology).cyan());
    let plans = resolver::resolve(&env, sandbox, &topology, true).await;

    for plan in plans {
        update_plan(plan).await;
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:#}", u::time_format(duration));
}

pub async fn update_component(
    profile: Option<String>,
    sandbox: Option<String>,
    component: Option<String>,
    recursive: bool,
) {
    let env = Env::maybe(profile);
    println!("Compiling topology");
    let topology = compiler::compile(&u::pwd(), recursive);

    println!("Resolving topology ({}) ", count_of(&topology).cyan());
    let plans = resolver::resolve(&env, sandbox, &topology, true).await;

    for plan in plans {
        deployer::update_component(plan.clone(), component.clone()).await;
    }
}

pub async fn delete(profile: Option<String>, sandbox: Option<String>, recursive: bool) {
    let env = Env::maybe(profile);
    println!("Compiling topology");
    let topology = compiler::compile(&u::pwd(), recursive);

    println!("Resolving topology ({}) ", count_of(&topology).cyan());
    let plans = resolver::resolve(&env, sandbox, &topology, false).await;

    for plan in plans {
        deployer::delete(plan).await
    }
}

pub async fn delete_component(
    profile: Option<String>,
    sandbox: Option<String>,
    component: Option<String>,
    recursive: bool,
) {
    let env = Env::maybe(profile);
    println!("Compiling topology");
    let topology = compiler::compile(&u::pwd(), recursive);

    println!("Resolving topology");
    let plans = resolver::resolve(&env, sandbox, &topology, false).await;

    for plan in plans {
        deployer::delete_component(plan, component.clone()).await
    }
}

pub async fn list(
    profile: Option<String>,
    sandbox: Option<String>,
    component: Option<String>,
    format: Option<String>,
) {
    let env = init_env(profile);
    if u::option_exists(component.clone()) {
        lister::list_component(&env, sandbox, component, format).await;
    } else {
        lister::list(&env, sandbox).await;
    }
}

pub async fn scaffold() {
    let dir = u::pwd();
    let kind = compiler::kind_of();
    match kind.as_ref() {
        "function" => {
            let function = compiler::current_function(&dir);
            match function {
                Some(f) => scaffolder::create_function(&f.name, &f.infra_dir).await,
                None => panic!("No function found"),
            }
        }
        "step-function" => {
            let functions = compiler::just_functions();
            for (_, f) in functions {
                scaffolder::create_function(&f.name, &f.infra_dir).await;
            }
        }
        _ => {
            let function = compiler::current_function(&dir);
            match function {
                Some(f) => scaffolder::create_function(&f.name, &f.infra_dir).await,
                None => panic!("No function found"),
            }
        }
    }
}

pub async fn bootstrap(
    profile: Option<String>,
    role_name: Option<String>,
    create: bool,
    delete: bool,
    show: bool,
) {
    let env = init_env(profile);
    match role_name {
        Some(role) => {
            if create {
                aws::bootstrap::create_role(&env, &role).await;
            } else if delete {
                aws::bootstrap::delete_role(&env, &role).await;
            } else if show {
                aws::bootstrap::show_role(&env, &role).await;
            } else {
                aws::bootstrap::show_role(&env, &role).await;
            }
        }
        None => println!("No role name given"),
    }
}

pub struct InvokeOptions {
    pub sandbox: Option<String>,
    pub mode: Option<String>,
    pub payload: Option<String>,
    pub name: Option<String>,
    pub kind: Option<String>,
    pub local: bool,
    pub dumb: bool,
}

pub async fn invoke(profile: Option<String>, opts: InvokeOptions) {
    let env = init_env(profile.clone());
    let InvokeOptions {
        sandbox,
        mode,
        payload,
        name,
        local,
        kind,
        dumb,
        ..
    } = opts;

    if local {
        invoker::run_local(payload).await;
    } else {
        let inferred_kind = compiler::kind_of();
        let kind = u::maybe_string(kind, &inferred_kind);
        let sandbox = resolver::as_sandbox(sandbox);

        invoker::invoke(&env, &sandbox, &kind, name, payload, mode, dumb).await;
    }
}

pub async fn emulate(profile: Option<String>, shell: bool) {
    let env = init_env(profile);
    let kind = compiler::kind_of();
    match kind.as_ref() {
        "step-function" => emulator::sfn().await,
        "function" => {
            if shell {
                emulator::shell(&env).await;
            } else {
                emulator::lambda(&env).await;
            }
        }
        _ => emulator::lambda(&env).await,
    }
}

pub async fn upgrade() {
    git::self_upgrade("tc", "").await
}

pub async fn init(profile: &Option<String>) {
    aws::init(profile).await;
    match std::env::var("TC_TRACE") {
        Ok(_) => kit::init_trace(),
        Err(_) => kit::init_log(),
    }
}
