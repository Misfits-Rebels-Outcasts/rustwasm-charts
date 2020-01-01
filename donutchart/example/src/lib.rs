use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

/*
How to create a SVG doughnut chart in Rust WebAssembly (rustwasm)?

This example illustrates how to create a SVG Doughnut/Donut Chart with Rust WebAssembly web-sys. 
The code is structured to be as simple and strait forward as possible. Basically it involves the following:

1. Create a figure element.
2. Add a svg element with a rect as background.
3. Create two svg circles and lay them over each other to create the effect of doughnut chart with a ring and a hole.
4. Create additional circles and use the stroke-array and stroke-offset behavior to act as the different segments of the donut chart. 
For example, the first circle is used to paint a donut chart segment representing 40% of the chart.
The second circe for another 20% of the chart in a different color. And finally, a third segment for the rest of the 40% in a different color.
5. Create a g element to group svg text that appears within the doughnut chart.
6. Add figcaption and ul list to provide a legend for the chart.
*/

pub fn rust_webassembly_svg_donut_chart()-> Result<(), JsValue>
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    
    let figure = document.create_element("figure")?;
    let div = document.create_element("div")
    .unwrap()
    .dyn_into::<web_sys::HtmlDivElement>()
    .unwrap();
    div.set_attribute("class","doughnut-main")?;
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    svg.set_attribute("width","300px")?;
    svg.set_attribute("height","300px")?;
    svg.set_attribute("viewBox","0 0 42 42")?;

    let rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
    rect.set_attribute("width","100%")?;
    rect.set_attribute("height","100%")?;
    rect.set_attribute("fill","white")?;
    let hole = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    hole.set_attribute("class","hole")?;
    hole.set_attribute("cx","21")?;
    hole.set_attribute("cy","21")?;
    hole.set_attribute("r","15.91549430918954")?;
    hole.set_attribute("fill","#fff")?;

    let ring = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    ring.set_attribute("class","ring")?;
    ring.set_attribute("cx","21")?;
    ring.set_attribute("cy","21")?;
    ring.set_attribute("r","15.91549430918954")?;
    ring.set_attribute("fill","transparent")?;
    ring.set_attribute("stroke","#d2d3d4")?;
    ring.set_attribute("stroke-width","3")?;

    let seg1 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    seg1.set_attribute("cx","21")?;
    seg1.set_attribute("cy","21")?;
    seg1.set_attribute("r","15.91549430918954")?;
    seg1.set_attribute("fill","transparent")?;
    seg1.set_attribute("stroke","#ce4b99")?;
    seg1.set_attribute("stroke-width","5")?;
    seg1.set_attribute("stroke-dasharray","40 60")?;
    seg1.set_attribute("stroke-dashoffset","25")?;

    let seg2 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    seg2.set_attribute("cx","21")?;
    seg2.set_attribute("cy","21")?;
    seg2.set_attribute("r","15.91549430918954")?;
    seg2.set_attribute("fill","transparent")?;
    seg2.set_attribute("stroke","#27A844")?;
    seg2.set_attribute("stroke-width","5")?;
    seg2.set_attribute("stroke-dasharray","20 80")?;
    seg2.set_attribute("stroke-dashoffset","85")?;

    let seg3 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    seg3.set_attribute("cx","21")?;
    seg3.set_attribute("cy","21")?;
    seg3.set_attribute("r","15.91549430918954")?;
    seg3.set_attribute("fill","transparent")?;
    seg3.set_attribute("stroke","#377bbc")?;
    seg3.set_attribute("stroke-width","5")?;
    seg3.set_attribute("stroke-dasharray","40 60")?;
    seg3.set_attribute("stroke-dashoffset","65")?;

    let g = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?;
    g.set_attribute("class","doughnut-text")?;

    let g_text1 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "text")?;
    g_text1.set_attribute("x","50%")?;
    g_text1.set_attribute("y","50%")?;
    g_text1.set_attribute("class","doughnut-number")?;
    g_text1.set_text_content(Some("100"));
    g.append_child(&g_text1)?;

    let g_text2 = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "text")?;
    g_text2.set_attribute("x","50%")?;
    g_text2.set_attribute("y","50%")?;
    g_text2.set_attribute("class","doughnut-label")?;
    g_text2.set_text_content(Some("Sales"));
    g.append_child(&g_text2)?;

    let figcaption = document.create_element("figcaption")?;
    figcaption.set_attribute("class","doughnut-key")?;

    let ul = document.create_element("ul")
    .unwrap()
    .dyn_into::<web_sys::HtmlUListElement>()
    .unwrap();

    ul.set_attribute("class","doughnut-key-list")?;
    ul.set_attribute("aria-hidden","true")?;

    ul.style()
    .set_property("list-style-type", "none")?;

    let li1 = document.create_element("li")?;
    let span1a = document.create_element("span")?;
    
    span1a.set_attribute("class","round-dot dot-red")?;
    let span1b = document.create_element("span")?;
    span1b.set_text_content(Some("App Store (40)"));                
    li1.append_child(&span1a).unwrap();
    li1.append_child(&span1b).unwrap();
    ul.append_child(&li1).unwrap();

    let li2 = document.create_element("li")?;
    let span2a = document.create_element("span")?;
    span2a.set_attribute("class","round-dot dot-green").unwrap();
    let span2b = document.create_element("span")?;
    span2b.set_text_content(Some("Website (20)"));                
    li2.append_child(&span2a).unwrap();
    li2.append_child(&span2b).unwrap();
    ul.append_child(&li2).unwrap();
    
    let li3 = document.create_element("li")?;
    let span3a = document.create_element("span")?;
    span3a.set_attribute("class","round-dot dot-blue").unwrap();
    let span3b = document.create_element("span")?;
    span3b.set_text_content(Some("Partners (40)"));                
    li3.append_child(&span3a).unwrap();
    li3.append_child(&span3b).unwrap();
    ul.append_child(&li3).unwrap();

    figcaption.append_child(&ul).unwrap();
    svg.append_child(&rect).unwrap();
    svg.append_child(&hole).unwrap();
    svg.append_child(&ring).unwrap();
    
    svg.append_child(&seg1).unwrap();
    svg.append_child(&seg2).unwrap();
    svg.append_child(&seg3).unwrap();
    
    svg.append_child(&g).unwrap();
    div.append_child(&svg).unwrap();
    figure.append_child(&div).unwrap();
    figure.append_child(&figcaption).unwrap();
    body.append_child(&figure).unwrap();

    Ok(())

              
}


#[wasm_bindgen(start)]
pub fn start() {
    rust_webassembly_svg_donut_chart().unwrap();
  

}