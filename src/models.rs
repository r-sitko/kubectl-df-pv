use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Summary {
    pods: Vec<PodStats>,
}

#[derive(Debug, Deserialize)]
pub struct PodStats {
    #[serde(rename = "podRef")]
    pod_ref: PodReference,
    #[serde(rename = "volume")]
    volume_stats: Option<Vec<VolumeStats>>,
}

#[derive(Debug, Deserialize)]
pub struct PodReference {
    name: String,
    namespace: String,
    uid: String,
}

#[derive(Debug, Deserialize)]
pub struct VolumeStats {
    #[serde(flatten)]
    fs_stats: FsStats,
    name: Option<String>,
    #[serde(rename = "pvcRef")]
    pvc_ref: Option<PVCReference>,
}

#[derive(Debug, Deserialize)]
pub struct FsStats {
    time: k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    #[serde(rename = "availableBytes")]
    available_bytes: u64,
    #[serde(rename = "capacityBytes")]
    capacity_bytes: u64,
    #[serde(rename = "usedBytes")]
    used_bytes: u64,
    #[serde(rename = "inodesFree")]
    inodes_free: u64,
    #[serde(rename = "inodes")]
    inodes: u64,
    #[serde(rename = "inodesUsed")]
    inodes_used: u64,
}

#[derive(Debug, Deserialize)]
pub struct PVCReference {
    name: String,
    namespace: String,
}
