use {
    std::string::ToString,
    serde::Serialize
};


#[derive(Serialize)]
pub struct GraphKind {
    graph: bool
}

impl GraphKind {
    pub(crate) fn new() -> Self {
        Self {
            graph: true
        }
    }
}

#[derive(Serialize)]
pub struct GraphNode {
    id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<String>
}

impl GraphNode {
    pub fn new<T: ToString>(id: T) -> Self {
        Self {
            id: id.to_string(),
            label: None,
            color: None,
            shape: None,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn id<T: ToString>(&mut self, id: T) -> &mut Self {
        self.id = id.to_string();
        self
    }

    pub fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn label<T: ToString>(&mut self, label: T) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn get_color(&self) -> Option<&String> {
        self.color.as_ref()
    }

    pub fn color<T: ToString>(&mut self, color: T) -> &mut Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn get_shape(&self) -> Option<&String> {
        self.shape.as_ref()
    }

    pub fn shape<T: Into<GraphNodeShape>>(&mut self, shape: T) -> &mut Self {
        self.label = Some(shape.into().to_string());
        self
    }
}

pub enum GraphNodeShape {
    Ellipse,
    Box
}

impl ToString for GraphNodeShape {
    fn to_string(&self) -> String {
        match self {
            GraphNodeShape::Ellipse => "ellipse".into(),
            GraphNodeShape::Box => "box".into(),
        }
    }
}

#[derive(Serialize)]

pub struct GraphEdge {
    from: String,
    to: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dashes: Option<bool>,
}

impl GraphEdge {
    pub fn new<T: ToString>(from: T, to: T) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            label: None,
            id: None,
            color: None,
            dashes: None,
        }
    }

    pub fn get_from(&self) -> &String {
        &self.from
    }

    pub fn from<T: ToString>(&mut self, from: T) -> &mut Self {
        self.from = from.to_string();
        self
    }

    pub fn get_to(&self) -> &String {
        &self.to
    }

    pub fn to<T: ToString>(&mut self, to: T) -> &mut Self {
        self.to = to.to_string();
        self
    }

    pub fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn label<T: ToString>(&mut self, label: T) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn id<T: ToString>(&mut self, id: T) -> &mut Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn get_color(&self) -> Option<&String> {
        self.color.as_ref()
    }

    pub fn color<T: ToString>(&mut self, color: T) -> &mut Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn get_dashes(&self) -> Option<bool> {
        self.dashes
    }

    pub fn dashes<T: Into<bool>>(&mut self, dashes: T) -> &mut Self {
        self.dashes = Some(dashes.into());
        self
    }
}
