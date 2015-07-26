#![allow(dead_code, non_camel_case_types, non_upper_case_globals, raw_pointer_derive)]
extern crate libc;
/* automatically generated by rust-bindgen */

pub type FcChar8 = ::libc::c_uchar;
pub type FcChar16 = ::libc::c_ushort;
pub type FcChar32 = ::libc::c_uint;
pub type FcBool = ::libc::c_int;
pub type Enum__FcType = ::libc::c_int;
pub const FcTypeUnknown: ::libc::c_int = -1;
pub const FcTypeVoid: ::libc::c_int = 0;
pub const FcTypeInteger: ::libc::c_int = 1;
pub const FcTypeDouble: ::libc::c_int = 2;
pub const FcTypeString: ::libc::c_int = 3;
pub const FcTypeBool: ::libc::c_int = 4;
pub const FcTypeMatrix: ::libc::c_int = 5;
pub const FcTypeCharSet: ::libc::c_int = 6;
pub const FcTypeFTFace: ::libc::c_int = 7;
pub const FcTypeLangSet: ::libc::c_int = 8;
pub type FcType = Enum__FcType;
#[repr(C)]
#[derive(Copy)]
pub struct _FcMatrix {
    pub xx: ::libc::c_double,
    pub xy: ::libc::c_double,
    pub yx: ::libc::c_double,
    pub yy: ::libc::c_double,
}
impl ::std::clone::Clone for _FcMatrix {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _FcMatrix {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type FcMatrix = _FcMatrix;
pub enum _FcCharSet { }
pub type FcCharSet = _FcCharSet;
#[repr(C)]
#[derive(Copy)]
pub struct _FcObjectType {
    pub object: *const ::libc::c_char,
    pub _type: FcType,
}
impl ::std::clone::Clone for _FcObjectType {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _FcObjectType {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type FcObjectType = _FcObjectType;
#[repr(C)]
#[derive(Copy)]
pub struct _FcConstant {
    pub name: *const FcChar8,
    pub object: *const ::libc::c_char,
    pub value: ::libc::c_int,
}
impl ::std::clone::Clone for _FcConstant {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _FcConstant {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type FcConstant = _FcConstant;
pub type Enum__FcResult = ::libc::c_uint;
pub const FcResultMatch: ::libc::c_uint = 0;
pub const FcResultNoMatch: ::libc::c_uint = 1;
pub const FcResultTypeMismatch: ::libc::c_uint = 2;
pub const FcResultNoId: ::libc::c_uint = 3;
pub const FcResultOutOfMemory: ::libc::c_uint = 4;
pub type FcResult = Enum__FcResult;
pub enum _FcPattern { }
pub type FcPattern = _FcPattern;
pub enum _FcLangSet { }
pub type FcLangSet = _FcLangSet;
#[repr(C)]
#[derive(Copy)]
pub struct _FcValue {
    pub _type: FcType,
    pub u: Union_Unnamed1,
}
impl ::std::clone::Clone for _FcValue {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _FcValue {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed1 {
    pub unsafe fn s(&mut self) -> *mut *const FcChar8 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn i(&mut self) -> *mut ::libc::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn b(&mut self) -> *mut FcBool {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn d(&mut self) -> *mut ::libc::c_double {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn m(&mut self) -> *mut *const FcMatrix {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn c(&mut self) -> *mut *const FcCharSet {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn f(&mut self) -> *mut *mut ::libc::c_void {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn l(&mut self) -> *mut *const FcLangSet {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type FcValue = _FcValue;
#[repr(C)]
#[derive(Copy)]
pub struct _FcFontSet {
    pub nfont: ::libc::c_int,
    pub sfont: ::libc::c_int,
    pub fonts: *mut *mut FcPattern,
}
impl ::std::clone::Clone for _FcFontSet {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _FcFontSet {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type FcFontSet = _FcFontSet;
#[repr(C)]
#[derive(Copy)]
pub struct _FcObjectSet {
    pub nobject: ::libc::c_int,
    pub sobject: ::libc::c_int,
    pub objects: *mut *const ::libc::c_char,
}
impl ::std::clone::Clone for _FcObjectSet {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for _FcObjectSet {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type FcObjectSet = _FcObjectSet;
pub type Enum__FcMatchKind = ::libc::c_uint;
pub const FcMatchPattern: ::libc::c_uint = 0;
pub const FcMatchFont: ::libc::c_uint = 1;
pub const FcMatchScan: ::libc::c_uint = 2;
pub type FcMatchKind = Enum__FcMatchKind;
pub type Enum__FcLangResult = ::libc::c_uint;
pub const FcLangEqual: ::libc::c_uint = 0;
pub const FcLangDifferentCountry: ::libc::c_uint = 1;
pub const FcLangDifferentTerritory: ::libc::c_uint = 1;
pub const FcLangDifferentLang: ::libc::c_uint = 2;
pub type FcLangResult = Enum__FcLangResult;
pub type Enum__FcSetName = ::libc::c_uint;
pub const FcSetSystem: ::libc::c_uint = 0;
pub const FcSetApplication: ::libc::c_uint = 1;
pub type FcSetName = Enum__FcSetName;
pub enum _FcAtomic { }
pub type FcAtomic = _FcAtomic;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const FcEndianBig: ::libc::c_uint = 0;
pub const FcEndianLittle: ::libc::c_uint = 1;
pub type FcEndian = Enum_Unnamed2;
pub enum _FcConfig { }
pub type FcConfig = _FcConfig;
pub enum _FcGlobalCache { }
pub type FcFileCache = _FcGlobalCache;
pub enum _FcBlanks { }
pub type FcBlanks = _FcBlanks;
pub enum _FcStrList { }
pub type FcStrList = _FcStrList;
pub enum _FcStrSet { }
pub type FcStrSet = _FcStrSet;
pub enum _FcCache { }
pub type FcCache = _FcCache;
extern "C" {
    pub fn FcBlanksCreate() -> *mut FcBlanks;
    pub fn FcBlanksDestroy(b: *mut FcBlanks) -> ();
    pub fn FcBlanksAdd(b: *mut FcBlanks, ucs4: FcChar32) -> FcBool;
    pub fn FcBlanksIsMember(b: *mut FcBlanks, ucs4: FcChar32) -> FcBool;
    pub fn FcCacheDir(c: *const FcCache) -> *const FcChar8;
    pub fn FcCacheCopySet(c: *const FcCache) -> *mut FcFontSet;
    pub fn FcCacheSubdir(c: *const FcCache, i: ::libc::c_int)
     -> *const FcChar8;
    pub fn FcCacheNumSubdir(c: *const FcCache) -> ::libc::c_int;
    pub fn FcCacheNumFont(c: *const FcCache) -> ::libc::c_int;
    pub fn FcDirCacheUnlink(dir: *const FcChar8, config: *mut FcConfig)
     -> FcBool;
    pub fn FcDirCacheValid(cache_file: *const FcChar8) -> FcBool;
    pub fn FcDirCacheClean(cache_dir: *const FcChar8, verbose: FcBool)
     -> FcBool;
    pub fn FcCacheCreateTagFile(config: *const FcConfig) -> ();
    pub fn FcConfigHome() -> *mut FcChar8;
    pub fn FcConfigEnableHome(enable: FcBool) -> FcBool;
    pub fn FcConfigFilename(url: *const FcChar8) -> *mut FcChar8;
    pub fn FcConfigCreate() -> *mut FcConfig;
    pub fn FcConfigReference(config: *mut FcConfig) -> *mut FcConfig;
    pub fn FcConfigDestroy(config: *mut FcConfig) -> ();
    pub fn FcConfigSetCurrent(config: *mut FcConfig) -> FcBool;
    pub fn FcConfigGetCurrent() -> *mut FcConfig;
    pub fn FcConfigUptoDate(config: *mut FcConfig) -> FcBool;
    pub fn FcConfigBuildFonts(config: *mut FcConfig) -> FcBool;
    pub fn FcConfigGetFontDirs(config: *mut FcConfig) -> *mut FcStrList;
    pub fn FcConfigGetConfigDirs(config: *mut FcConfig) -> *mut FcStrList;
    pub fn FcConfigGetConfigFiles(config: *mut FcConfig) -> *mut FcStrList;
    pub fn FcConfigGetCache(config: *mut FcConfig) -> *mut FcChar8;
    pub fn FcConfigGetBlanks(config: *mut FcConfig) -> *mut FcBlanks;
    pub fn FcConfigGetCacheDirs(config: *const FcConfig) -> *mut FcStrList;
    pub fn FcConfigGetRescanInterval(config: *mut FcConfig) -> ::libc::c_int;
    pub fn FcConfigSetRescanInterval(config: *mut FcConfig,
                                     rescanInterval: ::libc::c_int) -> FcBool;
    pub fn FcConfigGetFonts(config: *mut FcConfig, set: FcSetName)
     -> *mut FcFontSet;
    pub fn FcConfigAppFontAddFile(config: *mut FcConfig, file: *const FcChar8)
     -> FcBool;
    pub fn FcConfigAppFontAddDir(config: *mut FcConfig, dir: *const FcChar8)
     -> FcBool;
    pub fn FcConfigAppFontClear(config: *mut FcConfig) -> ();
    pub fn FcConfigSubstituteWithPat(config: *mut FcConfig, p: *mut FcPattern,
                                     p_pat: *mut FcPattern, kind: FcMatchKind)
     -> FcBool;
    pub fn FcConfigSubstitute(config: *mut FcConfig, p: *mut FcPattern,
                              kind: FcMatchKind) -> FcBool;
    pub fn FcConfigGetSysRoot(config: *const FcConfig) -> *const FcChar8;
    pub fn FcConfigSetSysRoot(config: *mut FcConfig, sysroot: *const FcChar8)
     -> ();
    pub fn FcCharSetCreate() -> *mut FcCharSet;
    pub fn FcCharSetNew() -> *mut FcCharSet;
    pub fn FcCharSetDestroy(fcs: *mut FcCharSet) -> ();
    pub fn FcCharSetAddChar(fcs: *mut FcCharSet, ucs4: FcChar32) -> FcBool;
    pub fn FcCharSetDelChar(fcs: *mut FcCharSet, ucs4: FcChar32) -> FcBool;
    pub fn FcCharSetCopy(src: *mut FcCharSet) -> *mut FcCharSet;
    pub fn FcCharSetEqual(a: *const FcCharSet, b: *const FcCharSet) -> FcBool;
    pub fn FcCharSetIntersect(a: *const FcCharSet, b: *const FcCharSet)
     -> *mut FcCharSet;
    pub fn FcCharSetUnion(a: *const FcCharSet, b: *const FcCharSet)
     -> *mut FcCharSet;
    pub fn FcCharSetSubtract(a: *const FcCharSet, b: *const FcCharSet)
     -> *mut FcCharSet;
    pub fn FcCharSetMerge(a: *mut FcCharSet, b: *const FcCharSet,
                          changed: *mut FcBool) -> FcBool;
    pub fn FcCharSetHasChar(fcs: *const FcCharSet, ucs4: FcChar32) -> FcBool;
    pub fn FcCharSetCount(a: *const FcCharSet) -> FcChar32;
    pub fn FcCharSetIntersectCount(a: *const FcCharSet, b: *const FcCharSet)
     -> FcChar32;
    pub fn FcCharSetSubtractCount(a: *const FcCharSet, b: *const FcCharSet)
     -> FcChar32;
    pub fn FcCharSetIsSubset(a: *const FcCharSet, b: *const FcCharSet)
     -> FcBool;
    pub fn FcCharSetFirstPage(a: *const FcCharSet, map: *mut FcChar32,
                              next: *mut FcChar32) -> FcChar32;
    pub fn FcCharSetNextPage(a: *const FcCharSet, map: *mut FcChar32,
                             next: *mut FcChar32) -> FcChar32;
    pub fn FcCharSetCoverage(a: *const FcCharSet, page: FcChar32,
                             result: *mut FcChar32) -> FcChar32;
    pub fn FcValuePrint(v: FcValue) -> ();
    pub fn FcPatternPrint(p: *const FcPattern) -> ();
    pub fn FcFontSetPrint(s: *const FcFontSet) -> ();
    pub fn FcGetDefaultLangs() -> *mut FcStrSet;
    pub fn FcDefaultSubstitute(pattern: *mut FcPattern) -> ();
    pub fn FcFileIsDir(file: *const FcChar8) -> FcBool;
    pub fn FcFileScan(set: *mut FcFontSet, dirs: *mut FcStrSet,
                      cache: *mut FcFileCache, blanks: *mut FcBlanks,
                      file: *const FcChar8, force: FcBool) -> FcBool;
    pub fn FcDirScan(set: *mut FcFontSet, dirs: *mut FcStrSet,
                     cache: *mut FcFileCache, blanks: *mut FcBlanks,
                     dir: *const FcChar8, force: FcBool) -> FcBool;
    pub fn FcDirSave(set: *mut FcFontSet, dirs: *mut FcStrSet,
                     dir: *const FcChar8) -> FcBool;
    pub fn FcDirCacheLoad(dir: *const FcChar8, config: *mut FcConfig,
                          cache_file: *mut *mut FcChar8) -> *mut FcCache;
    pub fn FcDirCacheRescan(dir: *const FcChar8, config: *mut FcConfig)
     -> *mut FcCache;
    pub fn FcDirCacheRead(dir: *const FcChar8, force: FcBool,
                          config: *mut FcConfig) -> *mut FcCache;
    pub fn FcDirCacheLoadFile(cache_file: *const FcChar8,
                              file_stat: *mut std::os::linux::raw::stat) -> *mut FcCache;
    pub fn FcDirCacheUnload(cache: *mut FcCache) -> ();
    pub fn FcFreeTypeQuery(file: *const FcChar8, id: ::libc::c_int,
                           blanks: *mut FcBlanks, count: *mut ::libc::c_int)
     -> *mut FcPattern;
    pub fn FcFontSetCreate() -> *mut FcFontSet;
    pub fn FcFontSetDestroy(s: *mut FcFontSet) -> ();
    pub fn FcFontSetAdd(s: *mut FcFontSet, font: *mut FcPattern) -> FcBool;
    pub fn FcInitLoadConfig() -> *mut FcConfig;
    pub fn FcInitLoadConfigAndFonts() -> *mut FcConfig;
    pub fn FcInit() -> FcBool;
    pub fn FcFini() -> ();
    pub fn FcGetVersion() -> ::libc::c_int;
    pub fn FcInitReinitialize() -> FcBool;
    pub fn FcInitBringUptoDate() -> FcBool;
    pub fn FcGetLangs() -> *mut FcStrSet;
    pub fn FcLangNormalize(lang: *const FcChar8) -> *mut FcChar8;
    pub fn FcLangGetCharSet(lang: *const FcChar8) -> *const FcCharSet;
    pub fn FcLangSetCreate() -> *mut FcLangSet;
    pub fn FcLangSetDestroy(ls: *mut FcLangSet) -> ();
    pub fn FcLangSetCopy(ls: *const FcLangSet) -> *mut FcLangSet;
    pub fn FcLangSetAdd(ls: *mut FcLangSet, lang: *const FcChar8) -> FcBool;
    pub fn FcLangSetDel(ls: *mut FcLangSet, lang: *const FcChar8) -> FcBool;
    pub fn FcLangSetHasLang(ls: *const FcLangSet, lang: *const FcChar8)
     -> FcLangResult;
    pub fn FcLangSetCompare(lsa: *const FcLangSet, lsb: *const FcLangSet)
     -> FcLangResult;
    pub fn FcLangSetContains(lsa: *const FcLangSet, lsb: *const FcLangSet)
     -> FcBool;
    pub fn FcLangSetEqual(lsa: *const FcLangSet, lsb: *const FcLangSet)
     -> FcBool;
    pub fn FcLangSetHash(ls: *const FcLangSet) -> FcChar32;
    pub fn FcLangSetGetLangs(ls: *const FcLangSet) -> *mut FcStrSet;
    pub fn FcLangSetUnion(a: *const FcLangSet, b: *const FcLangSet)
     -> *mut FcLangSet;
    pub fn FcLangSetSubtract(a: *const FcLangSet, b: *const FcLangSet)
     -> *mut FcLangSet;
    pub fn FcObjectSetCreate() -> *mut FcObjectSet;
    pub fn FcObjectSetAdd(os: *mut FcObjectSet, object: *const ::libc::c_char)
     -> FcBool;
    pub fn FcObjectSetDestroy(os: *mut FcObjectSet) -> ();
    pub fn FcObjectSetVaBuild(first: *const ::libc::c_char, ...)
     -> *mut FcObjectSet;
    pub fn FcObjectSetBuild(first: *const ::libc::c_char, ...)
     -> *mut FcObjectSet;
    pub fn FcFontSetList(config: *mut FcConfig, sets: *mut *mut FcFontSet,
                         nsets: ::libc::c_int, p: *mut FcPattern,
                         os: *mut FcObjectSet) -> *mut FcFontSet;
    pub fn FcFontList(config: *mut FcConfig, p: *mut FcPattern,
                      os: *mut FcObjectSet) -> *mut FcFontSet;
    pub fn FcAtomicCreate(file: *const FcChar8) -> *mut FcAtomic;
    pub fn FcAtomicLock(atomic: *mut FcAtomic) -> FcBool;
    pub fn FcAtomicNewFile(atomic: *mut FcAtomic) -> *mut FcChar8;
    pub fn FcAtomicOrigFile(atomic: *mut FcAtomic) -> *mut FcChar8;
    pub fn FcAtomicReplaceOrig(atomic: *mut FcAtomic) -> FcBool;
    pub fn FcAtomicDeleteNew(atomic: *mut FcAtomic) -> ();
    pub fn FcAtomicUnlock(atomic: *mut FcAtomic) -> ();
    pub fn FcAtomicDestroy(atomic: *mut FcAtomic) -> ();
    pub fn FcFontSetMatch(config: *mut FcConfig, sets: *mut *mut FcFontSet,
                          nsets: ::libc::c_int, p: *mut FcPattern,
                          result: *mut FcResult) -> *mut FcPattern;
    pub fn FcFontMatch(config: *mut FcConfig, p: *mut FcPattern,
                       result: *mut FcResult) -> *mut FcPattern;
    pub fn FcFontRenderPrepare(config: *mut FcConfig, pat: *mut FcPattern,
                               font: *mut FcPattern) -> *mut FcPattern;
    pub fn FcFontSetSort(config: *mut FcConfig, sets: *mut *mut FcFontSet,
                         nsets: ::libc::c_int, p: *mut FcPattern,
                         trim: FcBool, csp: *mut *mut FcCharSet,
                         result: *mut FcResult) -> *mut FcFontSet;
    pub fn FcFontSort(config: *mut FcConfig, p: *mut FcPattern, trim: FcBool,
                      csp: *mut *mut FcCharSet, result: *mut FcResult)
     -> *mut FcFontSet;
    pub fn FcFontSetSortDestroy(fs: *mut FcFontSet) -> ();
    pub fn FcMatrixCopy(mat: *const FcMatrix) -> *mut FcMatrix;
    pub fn FcMatrixEqual(mat1: *const FcMatrix, mat2: *const FcMatrix)
     -> FcBool;
    pub fn FcMatrixMultiply(result: *mut FcMatrix, a: *const FcMatrix,
                            b: *const FcMatrix) -> ();
    pub fn FcMatrixRotate(m: *mut FcMatrix, c: ::libc::c_double,
                          s: ::libc::c_double) -> ();
    pub fn FcMatrixScale(m: *mut FcMatrix, sx: ::libc::c_double,
                         sy: ::libc::c_double) -> ();
    pub fn FcMatrixShear(m: *mut FcMatrix, sh: ::libc::c_double,
                         sv: ::libc::c_double) -> ();
    pub fn FcNameRegisterObjectTypes(types: *const FcObjectType,
                                     ntype: ::libc::c_int) -> FcBool;
    pub fn FcNameUnregisterObjectTypes(types: *const FcObjectType,
                                       ntype: ::libc::c_int) -> FcBool;
    pub fn FcNameGetObjectType(object: *const ::libc::c_char)
     -> *const FcObjectType;
    pub fn FcNameRegisterConstants(consts: *const FcConstant,
                                   nconsts: ::libc::c_int) -> FcBool;
    pub fn FcNameUnregisterConstants(consts: *const FcConstant,
                                     nconsts: ::libc::c_int) -> FcBool;
    pub fn FcNameGetConstant(string: *const FcChar8) -> *const FcConstant;
    pub fn FcNameConstant(string: *const FcChar8, result: *mut ::libc::c_int)
     -> FcBool;
    pub fn FcNameParse(name: *const FcChar8) -> *mut FcPattern;
    pub fn FcNameUnparse(pat: *mut FcPattern) -> *mut FcChar8;
    pub fn FcPatternCreate() -> *mut FcPattern;
    pub fn FcPatternDuplicate(p: *const FcPattern) -> *mut FcPattern;
    pub fn FcPatternReference(p: *mut FcPattern) -> ();
    pub fn FcPatternFilter(p: *mut FcPattern, os: *const FcObjectSet)
     -> *mut FcPattern;
    pub fn FcValueDestroy(v: FcValue) -> ();
    pub fn FcValueEqual(va: FcValue, vb: FcValue) -> FcBool;
    pub fn FcValueSave(v: FcValue) -> FcValue;
    pub fn FcPatternDestroy(p: *mut FcPattern) -> ();
    pub fn FcPatternEqual(pa: *const FcPattern, pb: *const FcPattern)
     -> FcBool;
    pub fn FcPatternEqualSubset(pa: *const FcPattern, pb: *const FcPattern,
                                os: *const FcObjectSet) -> FcBool;
    pub fn FcPatternHash(p: *const FcPattern) -> FcChar32;
    pub fn FcPatternAdd(p: *mut FcPattern, object: *const ::libc::c_char,
                        value: FcValue, append: FcBool) -> FcBool;
    pub fn FcPatternAddWeak(p: *mut FcPattern, object: *const ::libc::c_char,
                            value: FcValue, append: FcBool) -> FcBool;
    pub fn FcPatternGet(p: *const FcPattern, object: *const ::libc::c_char,
                        id: ::libc::c_int, v: *mut FcValue) -> FcResult;
    pub fn FcPatternDel(p: *mut FcPattern, object: *const ::libc::c_char)
     -> FcBool;
    pub fn FcPatternRemove(p: *mut FcPattern, object: *const ::libc::c_char,
                           id: ::libc::c_int) -> FcBool;
    pub fn FcPatternAddInteger(p: *mut FcPattern,
                               object: *const ::libc::c_char,
                               i: ::libc::c_int) -> FcBool;
    pub fn FcPatternAddDouble(p: *mut FcPattern,
                              object: *const ::libc::c_char,
                              d: ::libc::c_double) -> FcBool;
    pub fn FcPatternAddString(p: *mut FcPattern,
                              object: *const ::libc::c_char,
                              s: *const FcChar8) -> FcBool;
    pub fn FcPatternAddMatrix(p: *mut FcPattern,
                              object: *const ::libc::c_char,
                              s: *const FcMatrix) -> FcBool;
    pub fn FcPatternAddCharSet(p: *mut FcPattern,
                               object: *const ::libc::c_char,
                               c: *const FcCharSet) -> FcBool;
    pub fn FcPatternAddBool(p: *mut FcPattern, object: *const ::libc::c_char,
                            b: FcBool) -> FcBool;
    pub fn FcPatternAddLangSet(p: *mut FcPattern,
                               object: *const ::libc::c_char,
                               ls: *const FcLangSet) -> FcBool;
    pub fn FcPatternGetInteger(p: *const FcPattern,
                               object: *const ::libc::c_char,
                               n: ::libc::c_int, i: *mut ::libc::c_int)
     -> FcResult;
    pub fn FcPatternGetDouble(p: *const FcPattern,
                              object: *const ::libc::c_char, n: ::libc::c_int,
                              d: *mut ::libc::c_double) -> FcResult;
    pub fn FcPatternGetString(p: *const FcPattern,
                              object: *const ::libc::c_char, n: ::libc::c_int,
                              s: *mut *mut FcChar8) -> FcResult;
    pub fn FcPatternGetMatrix(p: *const FcPattern,
                              object: *const ::libc::c_char, n: ::libc::c_int,
                              s: *mut *mut FcMatrix) -> FcResult;
    pub fn FcPatternGetCharSet(p: *const FcPattern,
                               object: *const ::libc::c_char,
                               n: ::libc::c_int, c: *mut *mut FcCharSet)
     -> FcResult;
    pub fn FcPatternGetBool(p: *const FcPattern,
                            object: *const ::libc::c_char, n: ::libc::c_int,
                            b: *mut FcBool) -> FcResult;
    pub fn FcPatternGetLangSet(p: *const FcPattern,
                               object: *const ::libc::c_char,
                               n: ::libc::c_int, ls: *mut *mut FcLangSet)
     -> FcResult;
    pub fn FcPatternVaBuild(p: *mut FcPattern, ...) -> *mut FcPattern;
    pub fn FcPatternBuild(p: *mut FcPattern, ...) -> *mut FcPattern;
    pub fn FcPatternFormat(pat: *mut FcPattern, format: *const FcChar8)
     -> *mut FcChar8;
    pub fn FcStrCopy(s: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrCopyFilename(s: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrPlus(s1: *const FcChar8, s2: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrFree(s: *mut FcChar8) -> ();
    pub fn FcStrDowncase(s: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrCmpIgnoreCase(s1: *const FcChar8, s2: *const FcChar8)
     -> ::libc::c_int;
    pub fn FcStrCmp(s1: *const FcChar8, s2: *const FcChar8) -> ::libc::c_int;
    pub fn FcStrStrIgnoreCase(s1: *const FcChar8, s2: *const FcChar8)
     -> *const FcChar8;
    pub fn FcStrStr(s1: *const FcChar8, s2: *const FcChar8) -> *const FcChar8;
    pub fn FcUtf8ToUcs4(src_orig: *const FcChar8, dst: *mut FcChar32,
                        len: ::libc::c_int) -> ::libc::c_int;
    pub fn FcUtf8Len(string: *const FcChar8, len: ::libc::c_int,
                     nchar: *mut ::libc::c_int, wchar: *mut ::libc::c_int)
     -> FcBool;
    pub fn FcUcs4ToUtf8(ucs4: FcChar32, dest: *mut FcChar8) -> ::libc::c_int;
    pub fn FcUtf16ToUcs4(src_orig: *const FcChar8, endian: FcEndian,
                         dst: *mut FcChar32, len: ::libc::c_int)
     -> ::libc::c_int;
    pub fn FcUtf16Len(string: *const FcChar8, endian: FcEndian,
                      len: ::libc::c_int, nchar: *mut ::libc::c_int,
                      wchar: *mut ::libc::c_int) -> FcBool;
    pub fn FcStrDirname(file: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrBasename(file: *const FcChar8) -> *mut FcChar8;
    pub fn FcStrSetCreate() -> *mut FcStrSet;
    pub fn FcStrSetMember(set: *mut FcStrSet, s: *const FcChar8) -> FcBool;
    pub fn FcStrSetEqual(sa: *mut FcStrSet, sb: *mut FcStrSet) -> FcBool;
    pub fn FcStrSetAdd(set: *mut FcStrSet, s: *const FcChar8) -> FcBool;
    pub fn FcStrSetAddFilename(set: *mut FcStrSet, s: *const FcChar8)
     -> FcBool;
    pub fn FcStrSetDel(set: *mut FcStrSet, s: *const FcChar8) -> FcBool;
    pub fn FcStrSetDestroy(set: *mut FcStrSet) -> ();
    pub fn FcStrListCreate(set: *mut FcStrSet) -> *mut FcStrList;
    pub fn FcStrListFirst(list: *mut FcStrList) -> ();
    pub fn FcStrListNext(list: *mut FcStrList) -> *mut FcChar8;
    pub fn FcStrListDone(list: *mut FcStrList) -> ();
    pub fn FcConfigParseAndLoad(config: *mut FcConfig, file: *const FcChar8,
                                complain: FcBool) -> FcBool;
}
