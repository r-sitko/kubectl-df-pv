use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Summary {
    pub pods: Vec<PodStats>,
}

#[derive(Debug, Deserialize)]
pub struct PodStats {
    #[serde(rename = "podRef")]
    pub pod_ref: PodReference,
    #[serde(rename = "volume")]
    pub volume_stats: Option<Vec<VolumeStats>>,
}

#[derive(Debug, Deserialize)]
pub struct PodReference {
    pub name: String,
    pub namespace: String,
    pub uid: String,
}

#[derive(Debug, Deserialize)]
pub struct VolumeStats {
    #[serde(flatten)]
    pub fs_stats: FsStats,
    pub name: Option<String>,
    #[serde(rename = "pvcRef")]
    pub pvc_ref: Option<PVCReference>,
}

#[derive(Debug, Deserialize)]
pub struct FsStats {
    pub time: k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    #[serde(rename = "availableBytes")]
    pub available_bytes: u64,
    #[serde(rename = "capacityBytes")]
    pub capacity_bytes: u64,
    #[serde(rename = "usedBytes")]
    pub used_bytes: u64,
    #[serde(rename = "inodesFree")]
    pub inodes_free: u64,
    #[serde(rename = "inodes")]
    pub inodes: u64,
    #[serde(rename = "inodesUsed")]
    pub inodes_used: u64,
}

#[derive(Debug, Deserialize)]
pub struct PVCReference {
    pub name: String,
    pub namespace: String,
}