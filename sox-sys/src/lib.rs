extern crate libc;

use libc::{c_char, c_uchar, c_int, c_uint, c_double, c_void, size_t};

const SOX_MAX_NLOOPS: uint = 8;

#[repr(C)]
#[deriving(Show,Clone,Copy)]
pub enum SoxErrorT {
  SOX_SUCCESS = 0,     /**< Function succeeded = 0 */
  SOX_EOF = -1,        /**< End Of File or other error = -1 */
  SOX_EHDR = 2000,     /**< Invalid Audio Header = 2000 */
  SOX_EFMT = 2001,     /**< Unsupported data format = 2001 */
  SOX_ENOMEM = 2002,   /**< Can't alloc memory = 2002 */
  SOX_EPERM = 2003,    /**< Operation not permitted = 2003 */
  SOX_ENOTSUP = 2004,  /**< Operation not supported = 2004 */
  SOX_EINVAL = 2005    /*< Invalid argument = 2005 */
}

#[repr(C)]
#[deriving(Show,Clone,Copy)]
pub enum SoxEncodingT {
    SOX_ENCODING_UNKNOWN    = 0,  /**< encoding has not yet been determined */

    SOX_ENCODING_SIGN2      = 1,  /**< signed linear 2's comp: Mac */
    SOX_ENCODING_UNSIGNED   = 2,  /**< unsigned linear: Sound Blaster */
    SOX_ENCODING_FLOAT      = 3,  /**< floating point (binary format) */
    SOX_ENCODING_FLOAT_TEXT = 4,  /**< floating point (text format) */
    SOX_ENCODING_FLAC       = 5,  /**< FLAC compression */
    SOX_ENCODING_HCOM       = 6,  /**< Mac FSSD files with Huffman compression */
    SOX_ENCODING_WAVPACK    = 7,  /**< WavPack with integer samples */
    SOX_ENCODING_WAVPACKF   = 8,  /**< WavPack with float samples */
    SOX_ENCODING_ULAW       = 9,  /**< u-law signed logs: US telephony, SPARC */
    SOX_ENCODING_ALAW       = 10, /**< A-law signed logs: non-US telephony, Psion */
    SOX_ENCODING_G721       = 11, /**< G.721 4-bit ADPCM */
    SOX_ENCODING_G723       = 12, /**< G.723 3 or 5 bit ADPCM */
    SOX_ENCODING_CL_ADPCM   = 13, /**< Creative Labs 8 --> 2,3,4 bit Compressed PCM */
    SOX_ENCODING_CL_ADPCM16 = 14, /**< Creative Labs 16 --> 4 bit Compressed PCM */
    SOX_ENCODING_MS_ADPCM   = 15, /**< Microsoft Compressed PCM */
    SOX_ENCODING_IMA_ADPCM  = 16, /**< IMA Compressed PCM */
    SOX_ENCODING_OKI_ADPCM  = 17, /**< Dialogic/OKI Compressed PCM */
    SOX_ENCODING_DPCM       = 18, /**< Differential PCM: Fasttracker 2 (xi) */
    SOX_ENCODING_DWVW       = 19, /**< Delta Width Variable Word */
    SOX_ENCODING_DWVWN      = 20, /**< Delta Width Variable Word N-bit */
    SOX_ENCODING_GSM        = 21, /**< GSM 6.10 33byte frame lossy compression */
    SOX_ENCODING_MP3        = 22, /**< MP3 compression */
    SOX_ENCODING_VORBIS     = 23, /**< Vorbis compression */
    SOX_ENCODING_AMR_WB     = 24, /**< AMR-WB compression */
    SOX_ENCODING_AMR_NB     = 25, /**< AMR-NB compression */
    SOX_ENCODING_CVSD       = 26, /**< Continuously Variable Slope Delta modulation */
    SOX_ENCODING_LPC10      = 27, /**< Linear Predictive Coding */
    SOX_ENCODING_OPUS       = 28, /**< Opus compression */

    SOX_ENCODINGS           = 29  /*< End of list marker */
}

#[repr(C)]
#[deriving(Clone,Copy)]
pub enum LsxIoType {
  LsxIoFile = 0,
  LsxIoPipe = 1,
  LsxIoUrl = 2
}

#[repr(C)]
#[deriving(Show,Clone,Copy)]
pub enum SoxBool {
    SoxFalse = 0,
    SoxTrue = 1
}

#[repr(C)]
#[deriving(Clone,Copy)]
pub enum SoxOptionT {
    SoxOptionNo = 0,
    SoxOptionYes = 1,
    SoxOptionDefault = 2
}

#[repr(C)]
#[deriving(Clone,Copy)]
pub struct SoxInstrInfoT {
    pub midi_note: c_char,
    pub midi_low: c_char,
    pub midi_hi: c_char,
    pub nloops: c_uchar,
    pub loops: c_uint
}

#[repr(C)]
#[deriving(Clone,Copy)]
pub struct SoxLoopInfoT {
    pub start: u64,
    pub length: u64,
    pub count: c_uint,
    pub _type: c_uchar
}

#[repr(C)]
#[deriving(Clone,Copy)]
#[allow(raw_pointer_deriving)]
pub struct SoxOobT {
    pub comments: *mut *const c_char,
    pub instr: SoxInstrInfoT,
    pub loops: [SoxLoopInfoT, ..SOX_MAX_NLOOPS]
}

#[repr(C)]
#[deriving(Clone,Copy)]
#[allow(raw_pointer_deriving)]
pub struct SoxSignalInfoT {
    pub rate: c_double,
    pub channels: c_uint,
    pub precision: c_uint,
    pub length: u64,
    pub mult: *mut c_double
}

#[repr(C)]
#[deriving(Clone,Copy)]
pub struct SoxEncodingInfoT {
    pub encoding: SoxEncodingT,
    pub bits_per_sample: c_uint,
    pub compression: c_double,
    pub reverse_bytes: SoxOptionT,
    pub reverse_nibbles: SoxOptionT,
    pub reverse_bits: SoxOptionT,
    pub opposite_endian: SoxBool
}

#[repr(C)]
#[deriving(Clone,Copy)]
#[allow(raw_pointer_deriving)]
pub struct SoxFormatHandlerT {
    pub sox_lib_version_code: c_uint,
    pub description: *const c_char,
    pub names: *const *const c_char,
    pub flags: c_uint,
    pub startread: *const c_void,
    pub read: *const c_void,
    pub stopread: *const c_void,
    pub startwrite: *const c_void,
    pub write: *const c_void,
    pub stopwrite: *const c_void,
    pub seek: *const c_void,
    pub write_formats: *const c_uint,
    pub write_rates: *const c_double,
    pub priv_size: size_t
}

#[repr(C)]
#[deriving(Copy)]
pub struct SoxErrStr([c_char, ..256]);
impl Clone for SoxErrStr {
    fn clone(&self) -> SoxErrStr {
        let mut vec_copy: [c_char, ..256] = [0, ..256];
        let &SoxErrStr(vec_orig) = self;
        for (elem_copy, elem_orig) in vec_copy.iter_mut().zip(vec_orig.iter()) {
            *elem_copy = elem_orig.clone();
        }
        SoxErrStr(vec_copy)
    }
}

#[repr(C)]
#[deriving(Clone,Copy)]
#[allow(raw_pointer_deriving)]
pub struct SoxFormatT {
    pub filename: *const c_char,
    pub signal: SoxSignalInfoT,
    pub encoding: SoxEncodingInfoT,
    pub filetype: *const c_char,
    pub oob: SoxOobT,
    pub seekable: SoxBool,
    pub mode: c_char,
    pub olength: u64,
    pub clips: u64,
    pub sox_errno: c_int,
    pub sox_errstr: SoxErrStr,
    pub fp: *mut c_void,
    pub io_type: LsxIoType,
    pub tell_off: u64,
    pub data_start: u64,
    pub handler: SoxFormatHandlerT,
    pub _priv: *mut c_void
}

extern {
    pub fn sox_version() -> *const c_char;
    pub fn sox_format_init() -> SoxErrorT;
    pub fn sox_open_read(
        path: *const c_char,
        signal: *const SoxSignalInfoT,
        encoding: *const SoxEncodingInfoT,
        filetype: *const c_char) -> *const SoxFormatT;
    pub fn sox_read(ft: *const SoxFormatT, buf: *const i32, len: size_t) -> size_t;
    pub fn sox_close(ft: *const SoxFormatT) -> SoxErrorT;
    //fn sox_quit() -> SoxErrorT;
}
