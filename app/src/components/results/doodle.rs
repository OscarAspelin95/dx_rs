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
fn SortableTable(table_props: Signal<TableProps>) -> Element {
    let sort_function = move |i: usize| async move {
        info!("{i}");

        let sort_by_header = &table_props.read().table_headers[i];

        info!("{:?}", sort_by_header);
    };

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
                                th {
                                    id: "table-header-row-item",
                                    onclick: move |_| async move {
                                        sort_function(i).await;
                                    },
                                    "{h}"
                                }
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
    let mut df = df!("column_1" => vec!["bla_c1", "bli_c1"],
                            "column_2" => vec!["bla_c2", "bli_c2"],
                            "column_3" => vec!["bla_c3", "bli_c3"],)
    .expect("Failed to create dataframe");

    let table_props = TableProps::from_df(Some("Caption".to_string()), &mut df);
    let props_signal = use_signal::<TableProps>(|| table_props);
    rsx! {
        SortableTable { table_props: props_signal }
    }
}
