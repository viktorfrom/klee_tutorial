use serde::Serialize;
use serde_json::value::{self, Map, Value as Json};

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[derive(Serialize)]
pub struct Res {
    id: String,
    rt: f32,
    wcet: f32,
    bt: f32,
    pre: f32,
}

fn format_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let rendered = format!("{} ", param.value().render(),);
    out.write(rendered.as_ref())?;
    Ok(())
}

pub fn make_data(
    tot_util: &f32,
    analysis: &Vec<(String, f32, f32, f32, f32)>,
) -> Map<String, Json> {
    let mut data = Map::new();
    let dt = chrono::offset::Utc::now();
    let mut result = vec![];

    for i in analysis {
        let res = Res {
            id: i.0.clone(),
            rt: i.1.clone(),
            wcet: i.2.clone(),
            bt: i.3.clone(),
            pre: i.4.clone(),
        };

        result.push(res);
    }

    data.insert("date".to_string(), to_json(dt.to_string()));
    data.insert("teams".to_string(), to_json(&result));
    data.insert("load".to_string(), to_json(&tot_util));
    data
}

pub fn render_file(
    tot_util: &f32,
    analysis: &Vec<(String, f32, f32, f32, f32)>,
) -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("format", Box::new(format_helper));

    let data = make_data(&tot_util, &analysis);

    handlebars
        .register_template_file("template", "./render_file/template.hbs")
        .unwrap();

    let mut output_file = File::create("target/srp_analysis.html")?;
    handlebars.render_to_write("template", &data, &mut output_file)?;
    println!("target/srp_analysis.html generated");
    Ok(())
}
