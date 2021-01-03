use serde::Serialize;
use serde_json::value::{self, Map, Value as Json};

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

// define a custom helper
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

// another custom helper
fn rank_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let rank = h
        .param(0)
        .and_then(|ref v| v.value().as_u64())
        .ok_or(RenderError::new(
            "Param 0 with u64 type is required for rank helper.",
        ))? as usize;
    let total = h
        .param(1)
        .as_ref()
        .and_then(|v| v.value().as_array())
        .map(|arr| arr.len())
        .ok_or(RenderError::new(
            "Param 1 with array type is required for rank helper",
        ))?;
    if rank == 0 {
        out.write("champion")?;
    } else if rank >= total - 2 {
        out.write("relegation")?;
    } else if rank <= 2 {
        out.write("acl")?;
    }
    Ok(())
}

// define some data
#[derive(Serialize)]
pub struct Team {
    name: String,
    pts: f32,
    pts1: f32,
    pts2: f32,
    pts3: f32,
}

// produce some data
pub fn make_data() -> Map<String, Json> {
    let mut data = Map::new();

    let dt = chrono::offset::Utc::now();
    data.insert("year".to_string(), to_json(dt.to_string()));

    let teams = vec![
        Team {
            name: "Jiangsu Suning".to_string(),
            pts: 43f32,
            pts1: 44f32,
            pts2: 45f32,
            pts3: 46f32,
        },
        Team {
            name: "Jiangsu Suning".to_string(),
            pts: 43f32,
            pts1: 44f32,
            pts2: 45f32,
            pts3: 46f32,
        },
    ];

    data.insert("teams".to_string(), to_json(&teams));
    data.insert("engine".to_string(), to_json("test"));
    data
}

pub fn render_file(
    tot_util: &f32,
    analysis: &Vec<(String, f32, f32, f32, f32)>,
) -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("format", Box::new(format_helper));
    handlebars.register_helper("ranking_label", Box::new(rank_helper));
    // handlebars.register_helper("format", Box::new(FORMAT_HELPER));

    let data = make_data();

    handlebars
        .register_template_file("template", "./render_file/template.hbs")
        .unwrap();

    let mut output_file = File::create("target/table.html")?;
    handlebars.render_to_write("template", &data, &mut output_file)?;
    println!("target/table.html generated");
    Ok(())
}
