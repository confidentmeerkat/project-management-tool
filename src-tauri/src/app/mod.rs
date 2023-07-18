use mongodb::bson::Document;
use mongodb::{
    options::ClientOptions,
    sync::{Client, Collection},
};
use tauri::command;

mod services;

struct ServiceContainer {
    project: services::ProjectService,
}

pub struct App {
    service_container: ServiceContainer,
}

impl App {
    pub fn new() -> App {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("project-management");

        let project_collection: Collection<Document> = db.collection("projects");
        let project_service = services::ProjectService::new(project_collection);

        App {
            service_container: ServiceContainer {
                project: project_service,
            },
        }
    }
}

#[command]
pub fn get_projects() -> Vec<Document> {
    let app = App::new();

    let result = app.service_container.project.index();

    return result.unwrap();
}
