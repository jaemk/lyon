use commands::FlattenCmd;
use lyon::svg::parser;
use lyon::path;
use lyon::path_builder::*;
use lyon::path_iterator::*;
use lyon::events::FlattenedEvent;
use std::io;

#[derive(Debug)]
pub enum FlattenError {
    Io(io::Error),
    Parse,
}

impl ::std::convert::From<::std::io::Error> for FlattenError {
    fn from(err: io::Error) -> Self { FlattenError::Io(err) }
}

pub fn flatten(mut cmd: FlattenCmd) -> Result<(), FlattenError> {
    let mut builder = path::Path::builder().with_svg();

    for item in parser::path::PathTokenizer::new(&cmd.input) {
        if let Ok(event) = item {
            builder.svg_event(event)
        } else {
            return Err(FlattenError::Parse);
        }
    }

    let path = builder.build();

    if cmd.count {
        let mut num_paths = 0;
        let mut num_vertices = 0;
        for event in path.path_iter().flattened(cmd.tolerance) {
            match event {
                FlattenedEvent::MoveTo(_) => {
                    num_vertices += 1;
                    num_paths += 1;
                }
                FlattenedEvent::LineTo(_) => {
                    num_vertices += 1;
                }
                FlattenedEvent::Close => {}
            }
        }

        try!{ writeln!(&mut *cmd.output, "vertices: {}", num_vertices) };
        try!{ writeln!(&mut *cmd.output, "paths: {}", num_paths) };

        return Ok(());
    }

    for event in path.path_iter().flattened(cmd.tolerance) {
        match event {
            FlattenedEvent::MoveTo(p) => {
                try!{ write!(&mut *cmd.output, "M {} {} ", p.x, p.y) };
            }
            FlattenedEvent::LineTo(p) => {
                try!{ write!(&mut *cmd.output, "L {} {} ", p.x, p.y) };
            }
            FlattenedEvent::Close => {
                try!{ write!(&mut *cmd.output, "Z") };
            }
        }
    }
    try!{ writeln!(&mut *cmd.output, "") };

    Ok(())
}
