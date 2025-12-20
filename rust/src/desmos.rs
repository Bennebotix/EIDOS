use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DesmosState {
    pub version: i32,
    #[serde(rename = "randomSeed")]
    pub random_seed: String,
    pub graph: GraphSettings,
    pub expressions: ExpressionList,
    #[serde(rename = "includeFunctionParametersInRandomSeed")]
    pub include_function_parameters_in_random_seed: bool,
    #[serde(rename = "doNotMigrateMovablePointStyle")]
    pub do_not_migrate_movable_point_style: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphSettings {
    pub viewport: Viewport,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Viewport {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExpressionList {
    pub list: Vec<Expression>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Expression {
    #[serde(rename = "expression")]
    Expression(ExpressionData),
    #[serde(rename = "text")]
    Text(TextData),
    #[serde(rename = "folder")]
    Folder(FolderData),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TextData {
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FolderData {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    pub hidden: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    pub collapsed: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExpressionData {
    pub id: String,
    pub color: String,
    pub latex: String,
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<bool>,
    #[serde(rename = "fillOpacity", skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<String>,
    #[serde(rename = "lineWidth", skip_serializing_if = "Option::is_none")]
    pub line_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
    #[serde(rename = "parametricDomain", skip_serializing_if = "Option::is_none")]
    pub parametric_domain: Option<Domain>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Domain {
    pub min: String,
    pub max: String,
}
