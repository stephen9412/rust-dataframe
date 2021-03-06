//! Data source evaluators and readers

use std::fs::File;
use std::{io::Read, rc::Rc};

use arrow::csv::{Reader as CsvReader, ReaderBuilder as CsvBuilder};
use arrow::{datatypes::SchemaRef, ipc::reader::FileReader as ArrowFileReader, record_batch::RecordBatch};
use parquet::arrow::{ArrowReader, ParquetFileArrowReader};
use parquet::file::reader::SerializedFileReader;

use crate::error::{DataFrameError, Result};
use crate::expression::{DataSourceType, Dataset, Reader, SqlDatabase, SortCriteria, BooleanFilter};
use crate::io::sql::postgres;
use crate::io::sql::SqlDataSource;

pub trait DataSourceEval {
    fn get_dataset(&self) -> Result<Dataset>;
}

impl DataSourceEval for Reader {
    fn get_dataset(&self) -> Result<Dataset> {
        use DataSourceType::*;
        use SqlDatabase::*;
        match &self.source {
            Csv(path, options) => {
                let mut builder = CsvBuilder::new()
                    .has_header(options.has_headers)
                    .infer_schema(options.max_records)
                    .with_batch_size(options.batch_size)
                    .with_delimiter(options.delimiter.unwrap_or(b','));
                if let Some(projection) = options.projection.clone() {
                    builder = builder.with_projection(projection);
                }
                // TODO set schema if user has set one
                let file = File::open(&path)?;
                let csv_reader = builder.build(file)?;
                let schema = csv_reader.schema();
                Ok(Dataset {
                    name: "csv_source".to_owned(),
                    columns: schema.fields().iter().map(|f| f.clone().into()).collect(),
                })
            }
            Json(path) => unimplemented!("JSON data source evaluation not yet implemented"),
            Parquet(path) => {
                let file = File::open(path)?;
                let file_reader = SerializedFileReader::new(file)?;
                let mut arrow_reader = ParquetFileArrowReader::new(Rc::new(file_reader));
                let schema = arrow_reader.get_schema()?;

                Ok(Dataset {
                    name: "parquet_file_source".to_owned(),
                    columns: schema.fields().iter().map(|f| f.clone().into()).collect(),
                })
            }
            Arrow(path) => {
                let file = File::open(&path)?;
                let reader = ArrowFileReader::try_new(file)?;
                Ok(Dataset {
                    name: "ipc_file_source".to_owned(),
                    columns: reader
                        .schema()
                        .fields()
                        .iter()
                        .map(|f| f.clone().into())
                        .collect(),
                })
            }
            Sql(table, options) => match options.db {
                Postgres => Ok(Dataset {
                    name: table.clone(),
                    columns: postgres::Postgres::get_table_schema(
                        options.connection_string.as_str(),
                        table.as_str(),
                    )?
                    .fields()
                    .iter()
                    .map(|f| f.clone().into())
                    .collect(),
                }),
                MsSql => unimplemented!("MSSQL data source not yet implemented"),
                MySql => unimplemented!("MySQL data source not yet implemented"),
            },
        }
    }
}

pub trait DataSource {
    fn get_dataset(&self) -> Result<Dataset>;
    fn source(&self) -> DataSourceType;
    fn format(&self) -> &str;
    fn schema(&self) -> arrow::datatypes::SchemaRef;
    fn next_batch(&mut self) -> Result<Option<RecordBatch>>;

    fn supports_projection(&self) -> bool {
        false
    }
    fn supports_filtering(&self) -> bool {
        false
    }
    fn supports_sorting(&self) -> bool {
        false
    }
    fn supports_limit(&self) -> bool {
        false
    }

    fn limit(&mut self, limit: usize) -> Result<()>;
    fn filter(&mut self, filter: BooleanFilter) -> Result<()>;
    fn project(&mut self, columns: Vec<String>) -> Result<()>;
    fn sort(&mut self, criteria: Vec<SortCriteria>) -> Result<()>;
}

pub struct CsvDataSource<R: Read> {
    path: String,
    options: CsvSourceOptions,
    projection: Vec<String>,
    limit: Option<usize>,
    read_schema: SchemaRef,
    projected_schema: SchemaRef,
    reader: arrow::csv::Reader<R>,
    
}

pub struct CsvSourceOptions {
    infer_schema: bool,
    read_schema: Option<SchemaRef>,
    has_header: bool,
    delimiter: Option<u8>,
    projection: Option<Vec<usize>>
}

impl<R: Read> DataSource for CsvDataSource<R> {
    
    fn get_dataset(&self) -> Result<Dataset> {
        todo!()
    }
    fn source(&self) -> DataSourceType {
        todo!()
    }
    fn format(&self) -> &str {
        "csv"
    }
    fn schema(&self) -> SchemaRef {
        todo!()
    }
    fn next_batch(&mut self) -> Result<Option<RecordBatch>> {
        todo!()
    }
    fn limit(&mut self, limit: usize) -> Result<()> {
        todo!()
    }
    fn filter(&mut self, filter: BooleanFilter) -> Result<()> {
        todo!()
    }
    fn project(&mut self, columns: Vec<String>) -> Result<()> {
        todo!()
    }
    fn sort(&mut self, criteria: Vec<SortCriteria>) -> Result<()> {
        todo!()
    }
    fn supports_projection(&self) -> bool {
        true
    }
    fn supports_filtering(&self) -> bool {
        false
    }
    fn supports_sorting(&self) -> bool {
        false
    }
    fn supports_limit(&self) -> bool {
        true
    }    
}