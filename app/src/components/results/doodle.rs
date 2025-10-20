use std::fmt::Debug;

use dioxus::prelude::*;
use polars::prelude::*;

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

impl TableProps {
    /// Convert a polars dataframe to a more appropriate form.
    fn from_df(caption: Option<String>, df: &mut DataFrame) -> Self {
        let mut rows: Vec<Vec<String>> = Vec::new();

        let columns: Vec<String> = df
            .get_column_names()
            .iter()
            .map(|n| n.to_string())
            .collect();

        let df_t = df.transpose(None, None).unwrap();

        df_t.iter().for_each(|row| {
            let row_vec: Vec<String> = row
                .iter()
                .map(|i| i.to_string().replace("\"", ""))
                .collect();
            println!("{:?}", &row_vec);

            rows.push(row_vec);
        });

        return Self {
            table_caption: caption,
            table_headers: columns,
            table_rows: rows,
        };
    }
}

#[component]
fn SortableTable(data: Signal<DataFrame>) -> Element {
    let mut df = data.read().clone();

    let table_props = use_signal(|| TableProps::new());
    // let table_props = TableProps::from_df(Some("Caption".to_string()), &mut df);

    // let sort_function = move |i: usize| async move {
    //     info!("{i}");

    //     let column_names: Vec<String> = df
    //         .get_column_names()
    //         .iter()
    //         .map(|n| n.to_string())
    //         .collect();

    //     info!("{:?}", column_names);
    // };

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
    let df = df!("column_1" => vec!["bla_c1", "bli_c1"],
                            "column_2" => vec!["bla_c2", "bli_c2"],
                            "column_3" => vec!["bla_c3", "bli_c3"],)
    .expect("Failed to create dataframe");

    // let table_props = TableProps::from_df(Some("Caption".to_string()), &mut df);

    let data = use_signal(|| df);
    // let props_signal = use_signal::<TableProps>(|| table_props);
    rsx! {
        SortableTable { data }
    }
}
