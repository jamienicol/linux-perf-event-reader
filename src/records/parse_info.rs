use crate::{AttrFlags, BranchSampleFormat, PerfEventAttr, ReadFormat, SampleFormat};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordParseInfo {
    pub sample_format: SampleFormat,
    pub branch_sample_format: BranchSampleFormat,
    pub read_format: ReadFormat,
    pub common_data_offset_from_end: Option<usize>,
    pub sample_regs_user: u64,
    pub regs_count: usize,
    pub nonsample_record_time_offset_from_end: Option<usize>,
    pub nonsample_record_id_offset_from_end: Option<usize>,
    pub sample_record_time_offset_from_start: Option<usize>,
    pub sample_record_id_offset_from_start: Option<usize>,
}

impl RecordParseInfo {
    pub fn from_attr(attr: &PerfEventAttr) -> Self {
        let sample_format = attr.sample_format;
        let branch_sample_format = attr.branch_sample_format;
        let read_format = attr.read_format;

        // struct sample_id {
        //     { u32 pid, tid; }   /* if PERF_SAMPLE_TID set */
        //     { u64 time;     }   /* if PERF_SAMPLE_TIME set */
        //     { u64 id;       }   /* if PERF_SAMPLE_ID set */
        //     { u64 stream_id;}   /* if PERF_SAMPLE_STREAM_ID set  */
        //     { u32 cpu, res; }   /* if PERF_SAMPLE_CPU set */
        //     { u64 id;       }   /* if PERF_SAMPLE_IDENTIFIER set */
        // };
        let common_data_offset_from_end = if attr.flags.contains(AttrFlags::SAMPLE_ID_ALL) {
            Some(
                sample_format
                    .intersection(
                        SampleFormat::TID
                            | SampleFormat::TIME
                            | SampleFormat::ID
                            | SampleFormat::STREAM_ID
                            | SampleFormat::CPU
                            | SampleFormat::IDENTIFIER,
                    )
                    .bits()
                    .count_ones() as usize
                    * 8,
            )
        } else {
            None
        };
        let sample_regs_user = attr.sample_regs_user;
        let regs_count = sample_regs_user.count_ones() as usize;
        let nonsample_record_time_offset_from_end = if attr.flags.contains(AttrFlags::SAMPLE_ID_ALL)
            && sample_format.contains(SampleFormat::TIME)
        {
            Some(
                sample_format
                    .intersection(
                        SampleFormat::TIME
                            | SampleFormat::ID
                            | SampleFormat::STREAM_ID
                            | SampleFormat::CPU
                            | SampleFormat::IDENTIFIER,
                    )
                    .bits()
                    .count_ones() as usize
                    * 8,
            )
        } else {
            None
        };
        let nonsample_record_id_offset_from_end = if attr.flags.contains(AttrFlags::SAMPLE_ID_ALL)
            && sample_format.intersects(SampleFormat::ID | SampleFormat::IDENTIFIER)
        {
            if sample_format.contains(SampleFormat::IDENTIFIER) {
                Some(8)
            } else {
                Some(
                    sample_format
                        .intersection(
                            SampleFormat::ID
                                | SampleFormat::STREAM_ID
                                | SampleFormat::CPU
                                | SampleFormat::IDENTIFIER,
                        )
                        .bits()
                        .count_ones() as usize
                        * 8,
                )
            }
        } else {
            None
        };

        // { u64 id;           } && PERF_SAMPLE_IDENTIFIER
        // { u64 ip;           } && PERF_SAMPLE_IP
        // { u32 pid; u32 tid; } && PERF_SAMPLE_TID
        // { u64 time;         } && PERF_SAMPLE_TIME
        // { u64 addr;         } && PERF_SAMPLE_ADDR
        // { u64 id;           } && PERF_SAMPLE_ID
        let sample_record_id_offset_from_start = if sample_format.contains(SampleFormat::IDENTIFIER)
        {
            Some(0)
        } else if sample_format.contains(SampleFormat::ID) {
            Some(
                sample_format
                    .intersection(
                        SampleFormat::IP
                            | SampleFormat::TID
                            | SampleFormat::TIME
                            | SampleFormat::ADDR,
                    )
                    .bits()
                    .count_ones() as usize
                    * 8,
            )
        } else {
            None
        };
        let sample_record_time_offset_from_start = if sample_format.contains(SampleFormat::TIME) {
            Some(
                sample_format
                    .intersection(SampleFormat::IDENTIFIER | SampleFormat::IP | SampleFormat::TID)
                    .bits()
                    .count_ones() as usize
                    * 8,
            )
        } else {
            None
        };

        Self {
            sample_format,
            branch_sample_format,
            read_format,
            common_data_offset_from_end,
            sample_regs_user,
            regs_count,
            nonsample_record_time_offset_from_end,
            nonsample_record_id_offset_from_end,
            sample_record_time_offset_from_start,
            sample_record_id_offset_from_start,
        }
    }
}