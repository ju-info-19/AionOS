// AionOS Agent Manager – PID 1, superviseur d'agents IA
use nix::sched::{clone, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult, Pid};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use tokio::sync::RwLock;
use warp::Filter;

type AgentMap = RwLock<HashMap<String, Pid>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("AionOS Agent Manager started (PID 1)");

    // Sert aussi l'API REST
    let agents: AgentMap = RwLock::new(HashMap::new());

    // Endpoint: lister les agents
    let list_route = warp::path("agents")
        .and(warp::get())
        .and(with_agents(agents.clone()))
        .map(|agents: AgentMap| {
            let list = agents.blocking_read().keys().cloned().collect::<Vec<_>>();
            warp::reply::json(&list)
        });

    // Endpoint: lancer un agent
    let run_route = warp::path("run")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_agents(agents.clone()))
        .map(|body: serde_json::Value, agents: AgentMap| {
            let path = body["path"].as_str().unwrap_or("/agents/default.elf");
            // Fork + exec dans un namespace
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child, .. }) => {
                    let name = path.split('/').last().unwrap_or("agent");
                    agents.blocking_write().insert(name.to_string(), child);
                    warp::reply::json(&serde_json::json!({"status": "started", "pid": child}))
                }
                Ok(ForkResult::Child) => {
                    let _ = Command::new(path).status();
                    std::process::exit(0);
                }
                Err(e) => {
                    warp::reply::json(&serde_json::json!({"error": format!("fork failed: {}", e)}))
                }
            }
        });

    let routes = list_route.or(run_route);
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn with_agents(agents: AgentMap) -> impl warp::Filter<Extract = (AgentMap,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || agents.clone())
}