/// File descriptor set used for reflection service.
pub const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("shared-service-descriptors");

tonic::include_proto!("bw.client");
tonic::include_proto!("bw.user");
