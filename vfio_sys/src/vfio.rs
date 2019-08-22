// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/* automatically generated by bindgen /usr/include/linux/vfio.h --constified-enum '*'
 * --with-derive-default --no-doc-comments --no-layout-tests */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;
pub const _IOC_NRMASK: u32 = 255;
pub const _IOC_TYPEMASK: u32 = 255;
pub const _IOC_SIZEMASK: u32 = 16383;
pub const _IOC_DIRMASK: u32 = 3;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;
pub const IOC_IN: u32 = 1073741824;
pub const IOC_OUT: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOCSIZE_MASK: u32 = 1073676288;
pub const IOCSIZE_SHIFT: u32 = 16;
pub const VFIO_API_VERSION: u32 = 0;
pub const VFIO_TYPE1_IOMMU: u32 = 1;
pub const VFIO_SPAPR_TCE_IOMMU: u32 = 2;
pub const VFIO_TYPE1v2_IOMMU: u32 = 3;
pub const VFIO_DMA_CC_IOMMU: u32 = 4;
pub const VFIO_EEH: u32 = 5;
pub const VFIO_TYPE1_NESTING_IOMMU: u32 = 6;
pub const VFIO_SPAPR_TCE_v2_IOMMU: u32 = 7;
pub const VFIO_NOIOMMU_IOMMU: u32 = 8;
pub const VFIO_TYPE: u32 = 59;
pub const VFIO_BASE: u32 = 100;
pub const VFIO_GROUP_FLAGS_VIABLE: u32 = 1;
pub const VFIO_GROUP_FLAGS_CONTAINER_SET: u32 = 2;
pub const VFIO_DEVICE_FLAGS_RESET: u32 = 1;
pub const VFIO_DEVICE_FLAGS_PCI: u32 = 2;
pub const VFIO_DEVICE_FLAGS_PLATFORM: u32 = 4;
pub const VFIO_DEVICE_FLAGS_AMBA: u32 = 8;
pub const VFIO_DEVICE_FLAGS_CCW: u32 = 16;
pub const VFIO_DEVICE_API_PCI_STRING: &'static [u8; 9usize] = b"vfio-pci\0";
pub const VFIO_DEVICE_API_PLATFORM_STRING: &'static [u8; 14usize] = b"vfio-platform\0";
pub const VFIO_DEVICE_API_AMBA_STRING: &'static [u8; 10usize] = b"vfio-amba\0";
pub const VFIO_DEVICE_API_CCW_STRING: &'static [u8; 9usize] = b"vfio-ccw\0";
pub const VFIO_REGION_INFO_FLAG_READ: u32 = 1;
pub const VFIO_REGION_INFO_FLAG_WRITE: u32 = 2;
pub const VFIO_REGION_INFO_FLAG_MMAP: u32 = 4;
pub const VFIO_REGION_INFO_FLAG_CAPS: u32 = 8;
pub const VFIO_REGION_INFO_CAP_SPARSE_MMAP: u32 = 1;
pub const VFIO_REGION_INFO_CAP_TYPE: u32 = 2;
pub const VFIO_REGION_TYPE_PCI_VENDOR_TYPE: u32 = 2147483648;
pub const VFIO_REGION_TYPE_PCI_VENDOR_MASK: u32 = 65535;
pub const VFIO_REGION_SUBTYPE_INTEL_IGD_OPREGION: u32 = 1;
pub const VFIO_REGION_SUBTYPE_INTEL_IGD_HOST_CFG: u32 = 2;
pub const VFIO_REGION_SUBTYPE_INTEL_IGD_LPC_CFG: u32 = 3;
pub const VFIO_REGION_INFO_CAP_MSIX_MAPPABLE: u32 = 3;
pub const VFIO_IRQ_INFO_EVENTFD: u32 = 1;
pub const VFIO_IRQ_INFO_MASKABLE: u32 = 2;
pub const VFIO_IRQ_INFO_AUTOMASKED: u32 = 4;
pub const VFIO_IRQ_INFO_NORESIZE: u32 = 8;
pub const VFIO_IRQ_SET_DATA_NONE: u32 = 1;
pub const VFIO_IRQ_SET_DATA_BOOL: u32 = 2;
pub const VFIO_IRQ_SET_DATA_EVENTFD: u32 = 4;
pub const VFIO_IRQ_SET_ACTION_MASK: u32 = 8;
pub const VFIO_IRQ_SET_ACTION_UNMASK: u32 = 16;
pub const VFIO_IRQ_SET_ACTION_TRIGGER: u32 = 32;
pub const VFIO_IRQ_SET_DATA_TYPE_MASK: u32 = 7;
pub const VFIO_IRQ_SET_ACTION_TYPE_MASK: u32 = 56;
pub const VFIO_GFX_PLANE_TYPE_PROBE: u32 = 1;
pub const VFIO_GFX_PLANE_TYPE_DMABUF: u32 = 2;
pub const VFIO_GFX_PLANE_TYPE_REGION: u32 = 4;
pub const VFIO_DEVICE_IOEVENTFD_8: u32 = 1;
pub const VFIO_DEVICE_IOEVENTFD_16: u32 = 2;
pub const VFIO_DEVICE_IOEVENTFD_32: u32 = 4;
pub const VFIO_DEVICE_IOEVENTFD_64: u32 = 8;
pub const VFIO_DEVICE_IOEVENTFD_SIZE_MASK: u32 = 15;
pub const VFIO_IOMMU_INFO_PGSIZES: u32 = 1;
pub const VFIO_DMA_MAP_FLAG_READ: u32 = 1;
pub const VFIO_DMA_MAP_FLAG_WRITE: u32 = 2;
pub const VFIO_IOMMU_SPAPR_INFO_DDW: u32 = 1;
pub const VFIO_EEH_PE_DISABLE: u32 = 0;
pub const VFIO_EEH_PE_ENABLE: u32 = 1;
pub const VFIO_EEH_PE_UNFREEZE_IO: u32 = 2;
pub const VFIO_EEH_PE_UNFREEZE_DMA: u32 = 3;
pub const VFIO_EEH_PE_GET_STATE: u32 = 4;
pub const VFIO_EEH_PE_STATE_NORMAL: u32 = 0;
pub const VFIO_EEH_PE_STATE_RESET: u32 = 1;
pub const VFIO_EEH_PE_STATE_STOPPED: u32 = 2;
pub const VFIO_EEH_PE_STATE_STOPPED_DMA: u32 = 4;
pub const VFIO_EEH_PE_STATE_UNAVAIL: u32 = 5;
pub const VFIO_EEH_PE_RESET_DEACTIVATE: u32 = 5;
pub const VFIO_EEH_PE_RESET_HOT: u32 = 6;
pub const VFIO_EEH_PE_RESET_FUNDAMENTAL: u32 = 7;
pub const VFIO_EEH_PE_CONFIGURE: u32 = 8;
pub const VFIO_EEH_PE_INJECT_ERR: u32 = 9;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_info_cap_header {
    pub id: __u16,
    pub version: __u16,
    pub next: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_group_status {
    pub argsz: __u32,
    pub flags: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_device_info {
    pub argsz: __u32,
    pub flags: __u32,
    pub num_regions: __u32,
    pub num_irqs: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_region_info {
    pub argsz: __u32,
    pub flags: __u32,
    pub index: __u32,
    pub cap_offset: __u32,
    pub size: __u64,
    pub offset: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_region_sparse_mmap_area {
    pub offset: __u64,
    pub size: __u64,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default)]
pub struct vfio_region_info_cap_sparse_mmap {
    pub header: vfio_info_cap_header,
    pub nr_areas: __u32,
    pub reserved: __u32,
    pub areas: __IncompleteArrayField<vfio_region_sparse_mmap_area>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_region_info_cap_type {
    pub header: vfio_info_cap_header,
    pub type_: __u32,
    pub subtype: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_irq_info {
    pub argsz: __u32,
    pub flags: __u32,
    pub index: __u32,
    pub count: __u32,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct vfio_irq_set {
    pub argsz: __u32,
    pub flags: __u32,
    pub index: __u32,
    pub start: __u32,
    pub count: __u32,
    pub data: __IncompleteArrayField<__u8>,
}
pub const VFIO_PCI_BAR0_REGION_INDEX: _bindgen_ty_1 = 0;
pub const VFIO_PCI_BAR1_REGION_INDEX: _bindgen_ty_1 = 1;
pub const VFIO_PCI_BAR2_REGION_INDEX: _bindgen_ty_1 = 2;
pub const VFIO_PCI_BAR3_REGION_INDEX: _bindgen_ty_1 = 3;
pub const VFIO_PCI_BAR4_REGION_INDEX: _bindgen_ty_1 = 4;
pub const VFIO_PCI_BAR5_REGION_INDEX: _bindgen_ty_1 = 5;
pub const VFIO_PCI_ROM_REGION_INDEX: _bindgen_ty_1 = 6;
pub const VFIO_PCI_CONFIG_REGION_INDEX: _bindgen_ty_1 = 7;
pub const VFIO_PCI_VGA_REGION_INDEX: _bindgen_ty_1 = 8;
pub const VFIO_PCI_NUM_REGIONS: _bindgen_ty_1 = 9;
pub type _bindgen_ty_1 = u32;
pub const VFIO_PCI_INTX_IRQ_INDEX: _bindgen_ty_2 = 0;
pub const VFIO_PCI_MSI_IRQ_INDEX: _bindgen_ty_2 = 1;
pub const VFIO_PCI_MSIX_IRQ_INDEX: _bindgen_ty_2 = 2;
pub const VFIO_PCI_ERR_IRQ_INDEX: _bindgen_ty_2 = 3;
pub const VFIO_PCI_REQ_IRQ_INDEX: _bindgen_ty_2 = 4;
pub const VFIO_PCI_NUM_IRQS: _bindgen_ty_2 = 5;
pub type _bindgen_ty_2 = u32;
pub const VFIO_CCW_CONFIG_REGION_INDEX: _bindgen_ty_3 = 0;
pub const VFIO_CCW_NUM_REGIONS: _bindgen_ty_3 = 1;
pub type _bindgen_ty_3 = u32;
pub const VFIO_CCW_IO_IRQ_INDEX: _bindgen_ty_4 = 0;
pub const VFIO_CCW_NUM_IRQS: _bindgen_ty_4 = 1;
pub type _bindgen_ty_4 = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_pci_dependent_device {
    pub group_id: __u32,
    pub segment: __u16,
    pub bus: __u8,
    pub devfn: __u8,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct vfio_pci_hot_reset_info {
    pub argsz: __u32,
    pub flags: __u32,
    pub count: __u32,
    pub devices: __IncompleteArrayField<vfio_pci_dependent_device>,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct vfio_pci_hot_reset {
    pub argsz: __u32,
    pub flags: __u32,
    pub count: __u32,
    pub group_fds: __IncompleteArrayField<__s32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device_gfx_plane_info {
    pub argsz: __u32,
    pub flags: __u32,
    pub drm_plane_type: __u32,
    pub drm_format: __u32,
    pub drm_format_mod: __u64,
    pub width: __u32,
    pub height: __u32,
    pub stride: __u32,
    pub size: __u32,
    pub x_pos: __u32,
    pub y_pos: __u32,
    pub x_hot: __u32,
    pub y_hot: __u32,
    pub __bindgen_anon_1: vfio_device_gfx_plane_info__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union vfio_device_gfx_plane_info__bindgen_ty_1 {
    pub region_index: __u32,
    pub dmabuf_id: __u32,
    _bindgen_union_align: u32,
}
impl Default for vfio_device_gfx_plane_info__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for vfio_device_gfx_plane_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_device_ioeventfd {
    pub argsz: __u32,
    pub flags: __u32,
    pub offset: __u64,
    pub data: __u64,
    pub fd: __s32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_iommu_type1_info {
    pub argsz: __u32,
    pub flags: __u32,
    pub iova_pgsizes: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_iommu_type1_dma_map {
    pub argsz: __u32,
    pub flags: __u32,
    pub vaddr: __u64,
    pub iova: __u64,
    pub size: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_iommu_type1_dma_unmap {
    pub argsz: __u32,
    pub flags: __u32,
    pub iova: __u64,
    pub size: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_iommu_spapr_tce_ddw_info {
    pub pgsizes: __u64,
    pub max_dynamic_windows_supported: __u32,
    pub levels: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_iommu_spapr_tce_info {
    pub argsz: __u32,
    pub flags: __u32,
    pub dma32_window_start: __u32,
    pub dma32_window_size: __u32,
    pub ddw: vfio_iommu_spapr_tce_ddw_info,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_eeh_pe_err {
    pub type_: __u32,
    pub func: __u32,
    pub addr: __u64,
    pub mask: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_eeh_pe_op {
    pub argsz: __u32,
    pub flags: __u32,
    pub op: __u32,
    pub __bindgen_anon_1: vfio_eeh_pe_op__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union vfio_eeh_pe_op__bindgen_ty_1 {
    pub err: vfio_eeh_pe_err,
    _bindgen_union_align: [u64; 3usize],
}
impl Default for vfio_eeh_pe_op__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for vfio_eeh_pe_op {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_iommu_spapr_register_memory {
    pub argsz: __u32,
    pub flags: __u32,
    pub vaddr: __u64,
    pub size: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_iommu_spapr_tce_create {
    pub argsz: __u32,
    pub flags: __u32,
    pub page_shift: __u32,
    pub __resv1: __u32,
    pub window_size: __u64,
    pub levels: __u32,
    pub __resv2: __u32,
    pub start_addr: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vfio_iommu_spapr_tce_remove {
    pub argsz: __u32,
    pub flags: __u32,
    pub start_addr: __u64,
}
