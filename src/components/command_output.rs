use serde_json::{Value};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Block;

use tui::widgets::{Paragraph, Wrap};
use tui::text::{Spans, Span};
use tui::Frame;

use structs::app::AppState;
use components::xml;

const DATA: &'static str = r#"
<Paragraph styles='{"wrap": {"trim": "true"}, "block": "default"}' scroll='true'>
    {{#each console_output_lines as |line| ~}}
        <Spans>
            <Span>{{line}}</Span>
        </Spans>
    {{/each}}
</Paragraph>"#;

fn get_buffer(area_height: u16, lines: Vec<Value>) -> Vec<Value> {
    let buffer_start = match area_height as usize <= lines.len() {
        false => 0,
        true => lines.len() - area_height as usize
    };
    (&lines[buffer_start..]).to_vec()
}

pub fn render<B>(frame: &mut Frame<B>, store: &AppState, area: Rect)
where
    B: Backend,
{
    // let array = store.json_store["console_output_lines"].as_array().expect("Data Error");

    // let buf = get_buffer(
    //     area.height, 
    //     array.to_vec()
    // );

    let dom_root = xml::parse(
        DATA.to_string(), 
        &store.json_store
    );

    let widget = match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => p,
        _ => panic!("XML Parse Error !")
    };

    // let text: Vec<Spans> = buf.iter()
    //     .map(|l| { 
    //         Spans::from(Span::raw(l.as_str().expect("Data Error"))) 
    //     }).collect();

    // let widget = Paragraph::new(text)
    //     .block(Block::default())
    //     .wrap(Wrap { trim: true });

    frame.render_widget(widget, area);
}
