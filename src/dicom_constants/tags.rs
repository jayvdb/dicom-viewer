use crate::dataset::tag::Tag;

pub const FILE_META_INFO_GROUP_LENGTH_TAG: Tag = Tag { group: 0x0002, element: 0x0000 };
pub const FILE_META_INFO_VERSION: Tag = Tag { group: 0x0002, element: 0x0001 };
pub const MEDIA_STORAGE_SOP_CLASS_UID: Tag = Tag { group: 0x0002, element: 0x0002 };
pub const MEDIA_STORAGE_SOP_INSTANCE_UID: Tag = Tag { group: 0x0002, element: 0x0003 };
pub const TRANSFER_SYNTAX_UID: Tag = Tag { group: 0x0002, element: 0x0010 };
pub const IMPLEMENTATION_CLASS_UID: Tag = Tag { group: 0x0002, element: 0x0012 };
pub const IMPLEMENTATION_VERSION_NAME: Tag = Tag { group: 0x0002, element: 0x0013 };
pub const TRANSFER_SYNTAX_UID_TAG: Tag = Tag { group: 0x0002, element: 0x0010 };
pub const IMPLEMENTATION_CLASS_UID_TAG: Tag = Tag { group: 0x0002, element: 0x0012 };
pub const IMPLEMENTATION_VERSION_NAME_TAG: Tag = Tag { group: 0x0002, element: 0x0013 };
pub const MODALITY: Tag = Tag { group: 0x0008, element: 0x0060 };
pub const SERIES_NUMBER: Tag = Tag { group: 0x0020, element: 0x0011 };
pub const INSTANCE_NUMBER: Tag = Tag { group: 0x0020, element: 0x0013 };

pub const ITEM: Tag = Tag { group: 0xFFFE, element: 0xE000 };
pub const ITEM_DELIMITATION: Tag = Tag { group: 0xFFFE, element: 0xE00D };
pub const SEQUENCE_DELIMITATION: Tag = Tag { group: 0xFFFE, element: 0xE0DD };
pub const STUDY_DATE: Tag = Tag { group: 0x0008, element: 0x0020 };
pub const STUDY_INSTANCE_UID: Tag = Tag { group: 0x0020, element: 0x000D };
pub const SERIES_INSTANCE_UID: Tag = Tag { group: 0x0020, element: 0x000E };
pub const IMAGE_POSITION: Tag = Tag { group: 0x0020, element: 0x0032 };
pub const IMAGE_ORIENTATION: Tag = Tag { group: 0x0020, element: 0x0037 };
pub const SAMPLES_PER_PIXEL: Tag = Tag { group: 0x0028, element: 0x0002 };
pub const PHOTOMETRIC_INTERPRETATION: Tag = Tag { group: 0x0028, element: 0x0004 };
pub const ROWS: Tag = Tag { group: 0x0028, element: 0x0010 };
pub const COLUMNS: Tag = Tag { group: 0x0028, element: 0x0011 };
pub const PIXEL_SPACING: Tag = Tag { group: 0x0028, element: 0x0030 };
pub const BITS_ALLOCATED: Tag = Tag { group: 0x0028, element: 0x0100 };
pub const BITS_STORED: Tag = Tag { group: 0x0028, element: 0x0101 };
pub const HIGH_BIT: Tag = Tag { group: 0x0028, element: 0x0102 };
pub const PIXEL_REPRESENTATION: Tag = Tag { group: 0x0028, element: 0x0103 };
pub const WINDOW_CENTER: Tag = Tag { group: 0x0028, element: 0x1050 };
pub const WINDOW_WIDTH: Tag = Tag { group: 0x0028, element: 0x1051 };
pub const RESCALE_INTERCEPT: Tag = Tag { group: 0x0028, element: 0x1052 };
pub const RESCALE_SLOPE: Tag = Tag { group: 0x0028, element: 0x1053 };
pub const RED_PALETTE_COLOR_LUT_DESCRIPTOR: Tag = Tag { group: 0x0028, element: 0x1101 };
pub const GREEN_PALETTE_COLOR_LUT_DESCRIPTOR: Tag = Tag { group: 0x0028, element: 0x1102 };
pub const BLUE_PALETTE_COLOR_LUT_DESCRIPTOR: Tag = Tag { group: 0x0028, element: 0x1103 };
pub const RED_PALETTE_COLOR_LUT_DATA: Tag = Tag { group: 0x0028, element: 0x1201 };
pub const GREEN_PALETTE_COLOR_LUT_DATA: Tag = Tag { group: 0x0028, element: 0x1202 };
pub const BLUE_PALETTE_COLOR_LUT_DATA: Tag = Tag { group: 0x0028, element: 0x1203 };
pub const PIXEL_DATA: Tag = Tag { group: 0x7FE0, element: 0x0010 };
