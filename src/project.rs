use std::fmt;
use std::str::FromStr;



/// Enum representing different types of projects
#[derive(Debug)]
pub enum ProjectType {
    FastAPI,
    Script,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProjectType::FastAPI => write!(f, "FastAPI"),
            ProjectType::Script => write!(f, "Script"),
        }
    }
}

impl FromStr for ProjectType {
    type Err = String;

    fn from_str(input: &str) -> Result<ProjectType, Self::Err> {
        match input.to_lowercase().as_str() {
            "fastapi" => Ok(ProjectType::FastAPI),
            "script" => Ok(ProjectType::Script),
            _ => Err(format!("'{}' is not a valid project type", input)),
        }
    }
}

impl Clone for ProjectType {
    fn clone(&self) -> ProjectType {
        match self {
            ProjectType::FastAPI => ProjectType::FastAPI,
            ProjectType::Script => ProjectType::Script,
        }
    }
}