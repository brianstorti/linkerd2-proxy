extern crate futures;
extern crate h2;
extern crate http;
extern crate prost;
extern crate prost_types;
extern crate tokio;
extern crate tower_grpc;
extern crate tracing;

pub mod span_exporter;

pub mod gen {
    pub mod agent {
        pub mod trace {
            pub mod v1 {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/opencensus.proto.agent.trace.v1.rs"
                ));
            }
        }
        pub mod common {
            pub mod v1 {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/opencensus.proto.agent.common.v1.rs"
                ));
            }
        }
    }
    pub mod trace {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/opencensus.proto.trace.v1.rs"));
        }
    }
    pub mod resource {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/opencensus.proto.resource.v1.rs"));
        }
    }
}
