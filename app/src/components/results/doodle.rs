use dioxus::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
struct TableProps {
    table_caption: Option<String>,
    table_headers: Vec<String>,
    table_rows: Vec<Vec<String>>,
}

impl TableProps {
    fn new() -> Self {
        Self {
            table_caption: None,
            table_headers: vec![],
            table_rows: vec![],
        }
    }
}

#[component]
fn SortableTable() -> Element {
    let table_props = use_signal(|| TableProps::new());

    rsx! {
        table { id: "test-table",
            // Optional table caption.
            if let Some(c) = &table_props.read().table_caption {
                caption { "{c}" }
            }

            thead { id: "table-header",
                tr { id: "table-header-row",
                    {
                        table_props
                            .read()
                            .table_headers
                            .iter()
                            .enumerate()
                            .map(|(i, h)| rsx! {
                                th { id: "table-header-row-item", onclick: move |_| async move {}, "{h}" }
                            })
                    }
                }
            }

            tbody { id: "table-body",
                {table_props.read().table_rows.iter().map(|r| rsx! {
                    tr { id: "table-body-row",
                        {r.iter().map(|row_row| rsx! {
                            td { id: "table-body-row-item", {format!("{:?}", row_row)} }
                        })}
                    }
                })}
            }
        }
    }
}

#[component]
pub fn Table() -> Element {
    rsx! {
        SortableTable {}
    }
}
