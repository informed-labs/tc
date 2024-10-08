use crate::resolver::Function;
use aws::lambda;
use aws::Env;
use kit as u;
use std::collections::HashMap;

pub async fn make_lambda(env: &Env, f: Function) -> lambda::Function {
    let client = lambda::make_client(env).await;
    let package_type = &f.runtime.package_type;

    let uri = u::unwrap(f.uri);

    let (size, blob, code) = lambda::make_code(package_type, &uri);
    let vpc_config = match f.runtime.network {
        Some(s) => Some(lambda::make_vpc_config(s.subnets, s.security_groups)),
        _ => None,
    };
    let filesystem_config = match f.fs {
        Some(s) => Some(vec![lambda::make_fs_config(&s.arn, &s.mount_point)]),
        _ => None,
    };

    let arch = lambda::make_arch(&f.runtime.lang);

    lambda::Function {
        client: client,
        name: f.name,
        actual_name: f.actual_name,
        description: f.description,
        code: code,
        code_size: size,
        blob: blob,
        role: f.role,
        runtime: lambda::make_runtime(&f.runtime.lang),
        handler: f.runtime.handler,
        timeout: f.runtime.timeout,
        uri: uri,
        memory_size: f.runtime.memory_size,
        package_type: lambda::make_package_type(package_type),
        environment: lambda::make_environment(f.runtime.environment),
        architecture: arch,
        tags: f.runtime.tags,
        layers: f.runtime.layers,
        vpc_config: vpc_config,
        filesystem_config: filesystem_config,
        logging_config: None,
    }
}

pub async fn create_function(profile: &str, f: Function) -> String {
    let env = Env::new(profile);
    match f.runtime.package_type.as_ref() {
        "zip" => {
            let lambda = make_lambda(&env, f.clone()).await;
            lambda.clone().create_or_update().await
        }
        _ => {
            let lambda = make_lambda(&env, f.clone()).await;
            lambda.clone().create_or_update().await
        }
    }
}

pub async fn create(profile: String, fns: HashMap<String, Function>) {
    let mut tasks = vec![];
    for (_, function) in fns {
        let p = profile.to_string();
        let h = tokio::spawn(async move {
            create_function(&p, function).await;
        });
        tasks.push(h);
    }
    for task in tasks {
        let _ = task.await;
    }
}

pub async fn update_code(profile: String, fns: HashMap<String, Function>) {
    let mut tasks = vec![];
    for (_, function) in fns {
        let p = profile.to_string();
        let h = tokio::spawn(async move {
            create_function(&p, function).await;
        });
        tasks.push(h);
    }
    for task in tasks {
        let _ = task.await;
    }
}

pub async fn delete_function(env: &Env, f: Function) {
    let function = make_lambda(env, f).await;
    function.clone().delete().await.unwrap();
}

pub async fn delete(env: &Env, fns: HashMap<String, Function>) {
    for (_name, function) in fns {
        match function.runtime.package_type.as_ref() {
            "zip" => {
                let function = make_lambda(env, function).await;
                function.clone().delete().await.unwrap();
            }
            _ => {
                let function = make_lambda(env, function).await;
                function.clone().delete().await.unwrap();
            }
        }
    }
}

pub async fn update_layers(env: &Env, fns: HashMap<String, Function>) {
    for (_, f) in fns {
        let function = make_lambda(env, f.clone()).await;
        let arn = env.lambda_arn(&f.name);
        let _ = function.update_layers(&arn).await;
    }
}

pub async fn update_vars(env: &Env, funcs: HashMap<String, Function>) {
    for (_, f) in funcs {
        let function = make_lambda(env, f.clone()).await;
        let _ = function.clone().update_vars().await;

        match f.runtime.provisioned_concurrency {
            Some(n) => function.update_concurrency(n).await,
            None => (),
        };
    }
}

pub async fn update_tags(env: &Env, funcs: HashMap<String, Function>) {
    let client = lambda::make_client(env).await;
    for (_, f) in funcs {
        let arn = env.lambda_arn(&f.name);
        lambda::update_tags(client.clone(), &f.name, &arn, f.runtime.tags.clone()).await;
    }
}
