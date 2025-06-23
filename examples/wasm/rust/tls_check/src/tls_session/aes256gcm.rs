



// Powers of x mod poly in GF(2).
const POWX: [u8; 16] = [
    0x01, 0x02, 0x04, 0x08,
    0x10, 0x20, 0x40, 0x80,
    0x1b, 0x36, 0x6c, 0xd8,
    0xab, 0x4d, 0x9a, 0x2f,
];


// FIPS-197 Figure 7. S-box substitution values in hexadecimal format.
const SBOX0: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

// FIPS-197 Figure 14.  Inverse S-box substitution values in hexadecimal format.
const SBOX1: [u8; 256] = [
	0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
	0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
	0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
	0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
	0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
	0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
	0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
	0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
	0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
	0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
	0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
	0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
	0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
	0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
	0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
	0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

// Lookup tables for encryption.
// These can be recomputed by adapting the tests in aes_test.go.

const TE0:[u32; 256] = [
	0xc66363a5, 0xf87c7c84, 0xee777799, 0xf67b7b8d, 0xfff2f20d, 0xd66b6bbd, 0xde6f6fb1, 0x91c5c554,
	0x60303050, 0x02010103, 0xce6767a9, 0x562b2b7d, 0xe7fefe19, 0xb5d7d762, 0x4dababe6, 0xec76769a,
	0x8fcaca45, 0x1f82829d, 0x89c9c940, 0xfa7d7d87, 0xeffafa15, 0xb25959eb, 0x8e4747c9, 0xfbf0f00b,
	0x41adadec, 0xb3d4d467, 0x5fa2a2fd, 0x45afafea, 0x239c9cbf, 0x53a4a4f7, 0xe4727296, 0x9bc0c05b,
	0x75b7b7c2, 0xe1fdfd1c, 0x3d9393ae, 0x4c26266a, 0x6c36365a, 0x7e3f3f41, 0xf5f7f702, 0x83cccc4f,
	0x6834345c, 0x51a5a5f4, 0xd1e5e534, 0xf9f1f108, 0xe2717193, 0xabd8d873, 0x62313153, 0x2a15153f,
	0x0804040c, 0x95c7c752, 0x46232365, 0x9dc3c35e, 0x30181828, 0x379696a1, 0x0a05050f, 0x2f9a9ab5,
	0x0e070709, 0x24121236, 0x1b80809b, 0xdfe2e23d, 0xcdebeb26, 0x4e272769, 0x7fb2b2cd, 0xea75759f,
	0x1209091b, 0x1d83839e, 0x582c2c74, 0x341a1a2e, 0x361b1b2d, 0xdc6e6eb2, 0xb45a5aee, 0x5ba0a0fb,
	0xa45252f6, 0x763b3b4d, 0xb7d6d661, 0x7db3b3ce, 0x5229297b, 0xdde3e33e, 0x5e2f2f71, 0x13848497,
	0xa65353f5, 0xb9d1d168, 0x00000000, 0xc1eded2c, 0x40202060, 0xe3fcfc1f, 0x79b1b1c8, 0xb65b5bed,
	0xd46a6abe, 0x8dcbcb46, 0x67bebed9, 0x7239394b, 0x944a4ade, 0x984c4cd4, 0xb05858e8, 0x85cfcf4a,
	0xbbd0d06b, 0xc5efef2a, 0x4faaaae5, 0xedfbfb16, 0x864343c5, 0x9a4d4dd7, 0x66333355, 0x11858594,
	0x8a4545cf, 0xe9f9f910, 0x04020206, 0xfe7f7f81, 0xa05050f0, 0x783c3c44, 0x259f9fba, 0x4ba8a8e3,
	0xa25151f3, 0x5da3a3fe, 0x804040c0, 0x058f8f8a, 0x3f9292ad, 0x219d9dbc, 0x70383848, 0xf1f5f504,
	0x63bcbcdf, 0x77b6b6c1, 0xafdada75, 0x42212163, 0x20101030, 0xe5ffff1a, 0xfdf3f30e, 0xbfd2d26d,
	0x81cdcd4c, 0x180c0c14, 0x26131335, 0xc3ecec2f, 0xbe5f5fe1, 0x359797a2, 0x884444cc, 0x2e171739,
	0x93c4c457, 0x55a7a7f2, 0xfc7e7e82, 0x7a3d3d47, 0xc86464ac, 0xba5d5de7, 0x3219192b, 0xe6737395,
	0xc06060a0, 0x19818198, 0x9e4f4fd1, 0xa3dcdc7f, 0x44222266, 0x542a2a7e, 0x3b9090ab, 0x0b888883,
	0x8c4646ca, 0xc7eeee29, 0x6bb8b8d3, 0x2814143c, 0xa7dede79, 0xbc5e5ee2, 0x160b0b1d, 0xaddbdb76,
	0xdbe0e03b, 0x64323256, 0x743a3a4e, 0x140a0a1e, 0x924949db, 0x0c06060a, 0x4824246c, 0xb85c5ce4,
	0x9fc2c25d, 0xbdd3d36e, 0x43acacef, 0xc46262a6, 0x399191a8, 0x319595a4, 0xd3e4e437, 0xf279798b,
	0xd5e7e732, 0x8bc8c843, 0x6e373759, 0xda6d6db7, 0x018d8d8c, 0xb1d5d564, 0x9c4e4ed2, 0x49a9a9e0,
	0xd86c6cb4, 0xac5656fa, 0xf3f4f407, 0xcfeaea25, 0xca6565af, 0xf47a7a8e, 0x47aeaee9, 0x10080818,
	0x6fbabad5, 0xf0787888, 0x4a25256f, 0x5c2e2e72, 0x381c1c24, 0x57a6a6f1, 0x73b4b4c7, 0x97c6c651,
	0xcbe8e823, 0xa1dddd7c, 0xe874749c, 0x3e1f1f21, 0x964b4bdd, 0x61bdbddc, 0x0d8b8b86, 0x0f8a8a85,
	0xe0707090, 0x7c3e3e42, 0x71b5b5c4, 0xcc6666aa, 0x904848d8, 0x06030305, 0xf7f6f601, 0x1c0e0e12,
	0xc26161a3, 0x6a35355f, 0xae5757f9, 0x69b9b9d0, 0x17868691, 0x99c1c158, 0x3a1d1d27, 0x279e9eb9,
	0xd9e1e138, 0xebf8f813, 0x2b9898b3, 0x22111133, 0xd26969bb, 0xa9d9d970, 0x078e8e89, 0x339494a7,
	0x2d9b9bb6, 0x3c1e1e22, 0x15878792, 0xc9e9e920, 0x87cece49, 0xaa5555ff, 0x50282878, 0xa5dfdf7a,
	0x038c8c8f, 0x59a1a1f8, 0x09898980, 0x1a0d0d17, 0x65bfbfda, 0xd7e6e631, 0x844242c6, 0xd06868b8,
	0x824141c3, 0x299999b0, 0x5a2d2d77, 0x1e0f0f11, 0x7bb0b0cb, 0xa85454fc, 0x6dbbbbd6, 0x2c16163a,
];

const TE1: [u32; 256] = [
	0xa5c66363, 0x84f87c7c, 0x99ee7777, 0x8df67b7b, 0x0dfff2f2, 0xbdd66b6b, 0xb1de6f6f, 0x5491c5c5,
	0x50603030, 0x03020101, 0xa9ce6767, 0x7d562b2b, 0x19e7fefe, 0x62b5d7d7, 0xe64dabab, 0x9aec7676,
	0x458fcaca, 0x9d1f8282, 0x4089c9c9, 0x87fa7d7d, 0x15effafa, 0xebb25959, 0xc98e4747, 0x0bfbf0f0,
	0xec41adad, 0x67b3d4d4, 0xfd5fa2a2, 0xea45afaf, 0xbf239c9c, 0xf753a4a4, 0x96e47272, 0x5b9bc0c0,
	0xc275b7b7, 0x1ce1fdfd, 0xae3d9393, 0x6a4c2626, 0x5a6c3636, 0x417e3f3f, 0x02f5f7f7, 0x4f83cccc,
	0x5c683434, 0xf451a5a5, 0x34d1e5e5, 0x08f9f1f1, 0x93e27171, 0x73abd8d8, 0x53623131, 0x3f2a1515,
	0x0c080404, 0x5295c7c7, 0x65462323, 0x5e9dc3c3, 0x28301818, 0xa1379696, 0x0f0a0505, 0xb52f9a9a,
	0x090e0707, 0x36241212, 0x9b1b8080, 0x3ddfe2e2, 0x26cdebeb, 0x694e2727, 0xcd7fb2b2, 0x9fea7575,
	0x1b120909, 0x9e1d8383, 0x74582c2c, 0x2e341a1a, 0x2d361b1b, 0xb2dc6e6e, 0xeeb45a5a, 0xfb5ba0a0,
	0xf6a45252, 0x4d763b3b, 0x61b7d6d6, 0xce7db3b3, 0x7b522929, 0x3edde3e3, 0x715e2f2f, 0x97138484,
	0xf5a65353, 0x68b9d1d1, 0x00000000, 0x2cc1eded, 0x60402020, 0x1fe3fcfc, 0xc879b1b1, 0xedb65b5b,
	0xbed46a6a, 0x468dcbcb, 0xd967bebe, 0x4b723939, 0xde944a4a, 0xd4984c4c, 0xe8b05858, 0x4a85cfcf,
	0x6bbbd0d0, 0x2ac5efef, 0xe54faaaa, 0x16edfbfb, 0xc5864343, 0xd79a4d4d, 0x55663333, 0x94118585,
	0xcf8a4545, 0x10e9f9f9, 0x06040202, 0x81fe7f7f, 0xf0a05050, 0x44783c3c, 0xba259f9f, 0xe34ba8a8,
	0xf3a25151, 0xfe5da3a3, 0xc0804040, 0x8a058f8f, 0xad3f9292, 0xbc219d9d, 0x48703838, 0x04f1f5f5,
	0xdf63bcbc, 0xc177b6b6, 0x75afdada, 0x63422121, 0x30201010, 0x1ae5ffff, 0x0efdf3f3, 0x6dbfd2d2,
	0x4c81cdcd, 0x14180c0c, 0x35261313, 0x2fc3ecec, 0xe1be5f5f, 0xa2359797, 0xcc884444, 0x392e1717,
	0x5793c4c4, 0xf255a7a7, 0x82fc7e7e, 0x477a3d3d, 0xacc86464, 0xe7ba5d5d, 0x2b321919, 0x95e67373,
	0xa0c06060, 0x98198181, 0xd19e4f4f, 0x7fa3dcdc, 0x66442222, 0x7e542a2a, 0xab3b9090, 0x830b8888,
	0xca8c4646, 0x29c7eeee, 0xd36bb8b8, 0x3c281414, 0x79a7dede, 0xe2bc5e5e, 0x1d160b0b, 0x76addbdb,
	0x3bdbe0e0, 0x56643232, 0x4e743a3a, 0x1e140a0a, 0xdb924949, 0x0a0c0606, 0x6c482424, 0xe4b85c5c,
	0x5d9fc2c2, 0x6ebdd3d3, 0xef43acac, 0xa6c46262, 0xa8399191, 0xa4319595, 0x37d3e4e4, 0x8bf27979,
	0x32d5e7e7, 0x438bc8c8, 0x596e3737, 0xb7da6d6d, 0x8c018d8d, 0x64b1d5d5, 0xd29c4e4e, 0xe049a9a9,
	0xb4d86c6c, 0xfaac5656, 0x07f3f4f4, 0x25cfeaea, 0xafca6565, 0x8ef47a7a, 0xe947aeae, 0x18100808,
	0xd56fbaba, 0x88f07878, 0x6f4a2525, 0x725c2e2e, 0x24381c1c, 0xf157a6a6, 0xc773b4b4, 0x5197c6c6,
	0x23cbe8e8, 0x7ca1dddd, 0x9ce87474, 0x213e1f1f, 0xdd964b4b, 0xdc61bdbd, 0x860d8b8b, 0x850f8a8a,
	0x90e07070, 0x427c3e3e, 0xc471b5b5, 0xaacc6666, 0xd8904848, 0x05060303, 0x01f7f6f6, 0x121c0e0e,
	0xa3c26161, 0x5f6a3535, 0xf9ae5757, 0xd069b9b9, 0x91178686, 0x5899c1c1, 0x273a1d1d, 0xb9279e9e,
	0x38d9e1e1, 0x13ebf8f8, 0xb32b9898, 0x33221111, 0xbbd26969, 0x70a9d9d9, 0x89078e8e, 0xa7339494,
	0xb62d9b9b, 0x223c1e1e, 0x92158787, 0x20c9e9e9, 0x4987cece, 0xffaa5555, 0x78502828, 0x7aa5dfdf,
	0x8f038c8c, 0xf859a1a1, 0x80098989, 0x171a0d0d, 0xda65bfbf, 0x31d7e6e6, 0xc6844242, 0xb8d06868,
	0xc3824141, 0xb0299999, 0x775a2d2d, 0x111e0f0f, 0xcb7bb0b0, 0xfca85454, 0xd66dbbbb, 0x3a2c1616,
];

const TE2:[u32; 256] = [
	0x63a5c663, 0x7c84f87c, 0x7799ee77, 0x7b8df67b, 0xf20dfff2, 0x6bbdd66b, 0x6fb1de6f, 0xc55491c5,
	0x30506030, 0x01030201, 0x67a9ce67, 0x2b7d562b, 0xfe19e7fe, 0xd762b5d7, 0xabe64dab, 0x769aec76,
	0xca458fca, 0x829d1f82, 0xc94089c9, 0x7d87fa7d, 0xfa15effa, 0x59ebb259, 0x47c98e47, 0xf00bfbf0,
	0xadec41ad, 0xd467b3d4, 0xa2fd5fa2, 0xafea45af, 0x9cbf239c, 0xa4f753a4, 0x7296e472, 0xc05b9bc0,
	0xb7c275b7, 0xfd1ce1fd, 0x93ae3d93, 0x266a4c26, 0x365a6c36, 0x3f417e3f, 0xf702f5f7, 0xcc4f83cc,
	0x345c6834, 0xa5f451a5, 0xe534d1e5, 0xf108f9f1, 0x7193e271, 0xd873abd8, 0x31536231, 0x153f2a15,
	0x040c0804, 0xc75295c7, 0x23654623, 0xc35e9dc3, 0x18283018, 0x96a13796, 0x050f0a05, 0x9ab52f9a,
	0x07090e07, 0x12362412, 0x809b1b80, 0xe23ddfe2, 0xeb26cdeb, 0x27694e27, 0xb2cd7fb2, 0x759fea75,
	0x091b1209, 0x839e1d83, 0x2c74582c, 0x1a2e341a, 0x1b2d361b, 0x6eb2dc6e, 0x5aeeb45a, 0xa0fb5ba0,
	0x52f6a452, 0x3b4d763b, 0xd661b7d6, 0xb3ce7db3, 0x297b5229, 0xe33edde3, 0x2f715e2f, 0x84971384,
	0x53f5a653, 0xd168b9d1, 0x00000000, 0xed2cc1ed, 0x20604020, 0xfc1fe3fc, 0xb1c879b1, 0x5bedb65b,
	0x6abed46a, 0xcb468dcb, 0xbed967be, 0x394b7239, 0x4ade944a, 0x4cd4984c, 0x58e8b058, 0xcf4a85cf,
	0xd06bbbd0, 0xef2ac5ef, 0xaae54faa, 0xfb16edfb, 0x43c58643, 0x4dd79a4d, 0x33556633, 0x85941185,
	0x45cf8a45, 0xf910e9f9, 0x02060402, 0x7f81fe7f, 0x50f0a050, 0x3c44783c, 0x9fba259f, 0xa8e34ba8,
	0x51f3a251, 0xa3fe5da3, 0x40c08040, 0x8f8a058f, 0x92ad3f92, 0x9dbc219d, 0x38487038, 0xf504f1f5,
	0xbcdf63bc, 0xb6c177b6, 0xda75afda, 0x21634221, 0x10302010, 0xff1ae5ff, 0xf30efdf3, 0xd26dbfd2,
	0xcd4c81cd, 0x0c14180c, 0x13352613, 0xec2fc3ec, 0x5fe1be5f, 0x97a23597, 0x44cc8844, 0x17392e17,
	0xc45793c4, 0xa7f255a7, 0x7e82fc7e, 0x3d477a3d, 0x64acc864, 0x5de7ba5d, 0x192b3219, 0x7395e673,
	0x60a0c060, 0x81981981, 0x4fd19e4f, 0xdc7fa3dc, 0x22664422, 0x2a7e542a, 0x90ab3b90, 0x88830b88,
	0x46ca8c46, 0xee29c7ee, 0xb8d36bb8, 0x143c2814, 0xde79a7de, 0x5ee2bc5e, 0x0b1d160b, 0xdb76addb,
	0xe03bdbe0, 0x32566432, 0x3a4e743a, 0x0a1e140a, 0x49db9249, 0x060a0c06, 0x246c4824, 0x5ce4b85c,
	0xc25d9fc2, 0xd36ebdd3, 0xacef43ac, 0x62a6c462, 0x91a83991, 0x95a43195, 0xe437d3e4, 0x798bf279,
	0xe732d5e7, 0xc8438bc8, 0x37596e37, 0x6db7da6d, 0x8d8c018d, 0xd564b1d5, 0x4ed29c4e, 0xa9e049a9,
	0x6cb4d86c, 0x56faac56, 0xf407f3f4, 0xea25cfea, 0x65afca65, 0x7a8ef47a, 0xaee947ae, 0x08181008,
	0xbad56fba, 0x7888f078, 0x256f4a25, 0x2e725c2e, 0x1c24381c, 0xa6f157a6, 0xb4c773b4, 0xc65197c6,
	0xe823cbe8, 0xdd7ca1dd, 0x749ce874, 0x1f213e1f, 0x4bdd964b, 0xbddc61bd, 0x8b860d8b, 0x8a850f8a,
	0x7090e070, 0x3e427c3e, 0xb5c471b5, 0x66aacc66, 0x48d89048, 0x03050603, 0xf601f7f6, 0x0e121c0e,
	0x61a3c261, 0x355f6a35, 0x57f9ae57, 0xb9d069b9, 0x86911786, 0xc15899c1, 0x1d273a1d, 0x9eb9279e,
	0xe138d9e1, 0xf813ebf8, 0x98b32b98, 0x11332211, 0x69bbd269, 0xd970a9d9, 0x8e89078e, 0x94a73394,
	0x9bb62d9b, 0x1e223c1e, 0x87921587, 0xe920c9e9, 0xce4987ce, 0x55ffaa55, 0x28785028, 0xdf7aa5df,
	0x8c8f038c, 0xa1f859a1, 0x89800989, 0x0d171a0d, 0xbfda65bf, 0xe631d7e6, 0x42c68442, 0x68b8d068,
	0x41c38241, 0x99b02999, 0x2d775a2d, 0x0f111e0f, 0xb0cb7bb0, 0x54fca854, 0xbbd66dbb, 0x163a2c16,
];

const TE3:[u32; 256] = [
	0x6363a5c6, 0x7c7c84f8, 0x777799ee, 0x7b7b8df6, 0xf2f20dff, 0x6b6bbdd6, 0x6f6fb1de, 0xc5c55491,
	0x30305060, 0x01010302, 0x6767a9ce, 0x2b2b7d56, 0xfefe19e7, 0xd7d762b5, 0xababe64d, 0x76769aec,
	0xcaca458f, 0x82829d1f, 0xc9c94089, 0x7d7d87fa, 0xfafa15ef, 0x5959ebb2, 0x4747c98e, 0xf0f00bfb,
	0xadadec41, 0xd4d467b3, 0xa2a2fd5f, 0xafafea45, 0x9c9cbf23, 0xa4a4f753, 0x727296e4, 0xc0c05b9b,
	0xb7b7c275, 0xfdfd1ce1, 0x9393ae3d, 0x26266a4c, 0x36365a6c, 0x3f3f417e, 0xf7f702f5, 0xcccc4f83,
	0x34345c68, 0xa5a5f451, 0xe5e534d1, 0xf1f108f9, 0x717193e2, 0xd8d873ab, 0x31315362, 0x15153f2a,
	0x04040c08, 0xc7c75295, 0x23236546, 0xc3c35e9d, 0x18182830, 0x9696a137, 0x05050f0a, 0x9a9ab52f,
	0x0707090e, 0x12123624, 0x80809b1b, 0xe2e23ddf, 0xebeb26cd, 0x2727694e, 0xb2b2cd7f, 0x75759fea,
	0x09091b12, 0x83839e1d, 0x2c2c7458, 0x1a1a2e34, 0x1b1b2d36, 0x6e6eb2dc, 0x5a5aeeb4, 0xa0a0fb5b,
	0x5252f6a4, 0x3b3b4d76, 0xd6d661b7, 0xb3b3ce7d, 0x29297b52, 0xe3e33edd, 0x2f2f715e, 0x84849713,
	0x5353f5a6, 0xd1d168b9, 0x00000000, 0xeded2cc1, 0x20206040, 0xfcfc1fe3, 0xb1b1c879, 0x5b5bedb6,
	0x6a6abed4, 0xcbcb468d, 0xbebed967, 0x39394b72, 0x4a4ade94, 0x4c4cd498, 0x5858e8b0, 0xcfcf4a85,
	0xd0d06bbb, 0xefef2ac5, 0xaaaae54f, 0xfbfb16ed, 0x4343c586, 0x4d4dd79a, 0x33335566, 0x85859411,
	0x4545cf8a, 0xf9f910e9, 0x02020604, 0x7f7f81fe, 0x5050f0a0, 0x3c3c4478, 0x9f9fba25, 0xa8a8e34b,
	0x5151f3a2, 0xa3a3fe5d, 0x4040c080, 0x8f8f8a05, 0x9292ad3f, 0x9d9dbc21, 0x38384870, 0xf5f504f1,
	0xbcbcdf63, 0xb6b6c177, 0xdada75af, 0x21216342, 0x10103020, 0xffff1ae5, 0xf3f30efd, 0xd2d26dbf,
	0xcdcd4c81, 0x0c0c1418, 0x13133526, 0xecec2fc3, 0x5f5fe1be, 0x9797a235, 0x4444cc88, 0x1717392e,
	0xc4c45793, 0xa7a7f255, 0x7e7e82fc, 0x3d3d477a, 0x6464acc8, 0x5d5de7ba, 0x19192b32, 0x737395e6,
	0x6060a0c0, 0x81819819, 0x4f4fd19e, 0xdcdc7fa3, 0x22226644, 0x2a2a7e54, 0x9090ab3b, 0x8888830b,
	0x4646ca8c, 0xeeee29c7, 0xb8b8d36b, 0x14143c28, 0xdede79a7, 0x5e5ee2bc, 0x0b0b1d16, 0xdbdb76ad,
	0xe0e03bdb, 0x32325664, 0x3a3a4e74, 0x0a0a1e14, 0x4949db92, 0x06060a0c, 0x24246c48, 0x5c5ce4b8,
	0xc2c25d9f, 0xd3d36ebd, 0xacacef43, 0x6262a6c4, 0x9191a839, 0x9595a431, 0xe4e437d3, 0x79798bf2,
	0xe7e732d5, 0xc8c8438b, 0x3737596e, 0x6d6db7da, 0x8d8d8c01, 0xd5d564b1, 0x4e4ed29c, 0xa9a9e049,
	0x6c6cb4d8, 0x5656faac, 0xf4f407f3, 0xeaea25cf, 0x6565afca, 0x7a7a8ef4, 0xaeaee947, 0x08081810,
	0xbabad56f, 0x787888f0, 0x25256f4a, 0x2e2e725c, 0x1c1c2438, 0xa6a6f157, 0xb4b4c773, 0xc6c65197,
	0xe8e823cb, 0xdddd7ca1, 0x74749ce8, 0x1f1f213e, 0x4b4bdd96, 0xbdbddc61, 0x8b8b860d, 0x8a8a850f,
	0x707090e0, 0x3e3e427c, 0xb5b5c471, 0x6666aacc, 0x4848d890, 0x03030506, 0xf6f601f7, 0x0e0e121c,
	0x6161a3c2, 0x35355f6a, 0x5757f9ae, 0xb9b9d069, 0x86869117, 0xc1c15899, 0x1d1d273a, 0x9e9eb927,
	0xe1e138d9, 0xf8f813eb, 0x9898b32b, 0x11113322, 0x6969bbd2, 0xd9d970a9, 0x8e8e8907, 0x9494a733,
	0x9b9bb62d, 0x1e1e223c, 0x87879215, 0xe9e920c9, 0xcece4987, 0x5555ffaa, 0x28287850, 0xdfdf7aa5,
	0x8c8c8f03, 0xa1a1f859, 0x89898009, 0x0d0d171a, 0xbfbfda65, 0xe6e631d7, 0x4242c684, 0x6868b8d0,
	0x4141c382, 0x9999b029, 0x2d2d775a, 0x0f0f111e, 0xb0b0cb7b, 0x5454fca8, 0xbbbbd66d, 0x16163a2c,
];

// Lookup tables for decryption.
// These can be recomputed by adapting the tests in aes_test.go.

const TD0:[u32; 256] = [
	0x51f4a750, 0x7e416553, 0x1a17a4c3, 0x3a275e96, 0x3bab6bcb, 0x1f9d45f1, 0xacfa58ab, 0x4be30393,
	0x2030fa55, 0xad766df6, 0x88cc7691, 0xf5024c25, 0x4fe5d7fc, 0xc52acbd7, 0x26354480, 0xb562a38f,
	0xdeb15a49, 0x25ba1b67, 0x45ea0e98, 0x5dfec0e1, 0xc32f7502, 0x814cf012, 0x8d4697a3, 0x6bd3f9c6,
	0x038f5fe7, 0x15929c95, 0xbf6d7aeb, 0x955259da, 0xd4be832d, 0x587421d3, 0x49e06929, 0x8ec9c844,
	0x75c2896a, 0xf48e7978, 0x99583e6b, 0x27b971dd, 0xbee14fb6, 0xf088ad17, 0xc920ac66, 0x7dce3ab4,
	0x63df4a18, 0xe51a3182, 0x97513360, 0x62537f45, 0xb16477e0, 0xbb6bae84, 0xfe81a01c, 0xf9082b94,
	0x70486858, 0x8f45fd19, 0x94de6c87, 0x527bf8b7, 0xab73d323, 0x724b02e2, 0xe31f8f57, 0x6655ab2a,
	0xb2eb2807, 0x2fb5c203, 0x86c57b9a, 0xd33708a5, 0x302887f2, 0x23bfa5b2, 0x02036aba, 0xed16825c,
	0x8acf1c2b, 0xa779b492, 0xf307f2f0, 0x4e69e2a1, 0x65daf4cd, 0x0605bed5, 0xd134621f, 0xc4a6fe8a,
	0x342e539d, 0xa2f355a0, 0x058ae132, 0xa4f6eb75, 0x0b83ec39, 0x4060efaa, 0x5e719f06, 0xbd6e1051,
	0x3e218af9, 0x96dd063d, 0xdd3e05ae, 0x4de6bd46, 0x91548db5, 0x71c45d05, 0x0406d46f, 0x605015ff,
	0x1998fb24, 0xd6bde997, 0x894043cc, 0x67d99e77, 0xb0e842bd, 0x07898b88, 0xe7195b38, 0x79c8eedb,
	0xa17c0a47, 0x7c420fe9, 0xf8841ec9, 0x00000000, 0x09808683, 0x322bed48, 0x1e1170ac, 0x6c5a724e,
	0xfd0efffb, 0x0f853856, 0x3daed51e, 0x362d3927, 0x0a0fd964, 0x685ca621, 0x9b5b54d1, 0x24362e3a,
	0x0c0a67b1, 0x9357e70f, 0xb4ee96d2, 0x1b9b919e, 0x80c0c54f, 0x61dc20a2, 0x5a774b69, 0x1c121a16,
	0xe293ba0a, 0xc0a02ae5, 0x3c22e043, 0x121b171d, 0x0e090d0b, 0xf28bc7ad, 0x2db6a8b9, 0x141ea9c8,
	0x57f11985, 0xaf75074c, 0xee99ddbb, 0xa37f60fd, 0xf701269f, 0x5c72f5bc, 0x44663bc5, 0x5bfb7e34,
	0x8b432976, 0xcb23c6dc, 0xb6edfc68, 0xb8e4f163, 0xd731dcca, 0x42638510, 0x13972240, 0x84c61120,
	0x854a247d, 0xd2bb3df8, 0xaef93211, 0xc729a16d, 0x1d9e2f4b, 0xdcb230f3, 0x0d8652ec, 0x77c1e3d0,
	0x2bb3166c, 0xa970b999, 0x119448fa, 0x47e96422, 0xa8fc8cc4, 0xa0f03f1a, 0x567d2cd8, 0x223390ef,
	0x87494ec7, 0xd938d1c1, 0x8ccaa2fe, 0x98d40b36, 0xa6f581cf, 0xa57ade28, 0xdab78e26, 0x3fadbfa4,
	0x2c3a9de4, 0x5078920d, 0x6a5fcc9b, 0x547e4662, 0xf68d13c2, 0x90d8b8e8, 0x2e39f75e, 0x82c3aff5,
	0x9f5d80be, 0x69d0937c, 0x6fd52da9, 0xcf2512b3, 0xc8ac993b, 0x10187da7, 0xe89c636e, 0xdb3bbb7b,
	0xcd267809, 0x6e5918f4, 0xec9ab701, 0x834f9aa8, 0xe6956e65, 0xaaffe67e, 0x21bccf08, 0xef15e8e6,
	0xbae79bd9, 0x4a6f36ce, 0xea9f09d4, 0x29b07cd6, 0x31a4b2af, 0x2a3f2331, 0xc6a59430, 0x35a266c0,
	0x744ebc37, 0xfc82caa6, 0xe090d0b0, 0x33a7d815, 0xf104984a, 0x41ecdaf7, 0x7fcd500e, 0x1791f62f,
	0x764dd68d, 0x43efb04d, 0xccaa4d54, 0xe49604df, 0x9ed1b5e3, 0x4c6a881b, 0xc12c1fb8, 0x4665517f,
	0x9d5eea04, 0x018c355d, 0xfa877473, 0xfb0b412e, 0xb3671d5a, 0x92dbd252, 0xe9105633, 0x6dd64713,
	0x9ad7618c, 0x37a10c7a, 0x59f8148e, 0xeb133c89, 0xcea927ee, 0xb761c935, 0xe11ce5ed, 0x7a47b13c,
	0x9cd2df59, 0x55f2733f, 0x1814ce79, 0x73c737bf, 0x53f7cdea, 0x5ffdaa5b, 0xdf3d6f14, 0x7844db86,
	0xcaaff381, 0xb968c43e, 0x3824342c, 0xc2a3405f, 0x161dc372, 0xbce2250c, 0x283c498b, 0xff0d9541,
	0x39a80171, 0x080cb3de, 0xd8b4e49c, 0x6456c190, 0x7bcb8461, 0xd532b670, 0x486c5c74, 0xd0b85742,
];

const TD1:[u32; 256] = [
	0x5051f4a7, 0x537e4165, 0xc31a17a4, 0x963a275e, 0xcb3bab6b, 0xf11f9d45, 0xabacfa58, 0x934be303,
	0x552030fa, 0xf6ad766d, 0x9188cc76, 0x25f5024c, 0xfc4fe5d7, 0xd7c52acb, 0x80263544, 0x8fb562a3,
	0x49deb15a, 0x6725ba1b, 0x9845ea0e, 0xe15dfec0, 0x02c32f75, 0x12814cf0, 0xa38d4697, 0xc66bd3f9,
	0xe7038f5f, 0x9515929c, 0xebbf6d7a, 0xda955259, 0x2dd4be83, 0xd3587421, 0x2949e069, 0x448ec9c8,
	0x6a75c289, 0x78f48e79, 0x6b99583e, 0xdd27b971, 0xb6bee14f, 0x17f088ad, 0x66c920ac, 0xb47dce3a,
	0x1863df4a, 0x82e51a31, 0x60975133, 0x4562537f, 0xe0b16477, 0x84bb6bae, 0x1cfe81a0, 0x94f9082b,
	0x58704868, 0x198f45fd, 0x8794de6c, 0xb7527bf8, 0x23ab73d3, 0xe2724b02, 0x57e31f8f, 0x2a6655ab,
	0x07b2eb28, 0x032fb5c2, 0x9a86c57b, 0xa5d33708, 0xf2302887, 0xb223bfa5, 0xba02036a, 0x5ced1682,
	0x2b8acf1c, 0x92a779b4, 0xf0f307f2, 0xa14e69e2, 0xcd65daf4, 0xd50605be, 0x1fd13462, 0x8ac4a6fe,
	0x9d342e53, 0xa0a2f355, 0x32058ae1, 0x75a4f6eb, 0x390b83ec, 0xaa4060ef, 0x065e719f, 0x51bd6e10,
	0xf93e218a, 0x3d96dd06, 0xaedd3e05, 0x464de6bd, 0xb591548d, 0x0571c45d, 0x6f0406d4, 0xff605015,
	0x241998fb, 0x97d6bde9, 0xcc894043, 0x7767d99e, 0xbdb0e842, 0x8807898b, 0x38e7195b, 0xdb79c8ee,
	0x47a17c0a, 0xe97c420f, 0xc9f8841e, 0x00000000, 0x83098086, 0x48322bed, 0xac1e1170, 0x4e6c5a72,
	0xfbfd0eff, 0x560f8538, 0x1e3daed5, 0x27362d39, 0x640a0fd9, 0x21685ca6, 0xd19b5b54, 0x3a24362e,
	0xb10c0a67, 0x0f9357e7, 0xd2b4ee96, 0x9e1b9b91, 0x4f80c0c5, 0xa261dc20, 0x695a774b, 0x161c121a,
	0x0ae293ba, 0xe5c0a02a, 0x433c22e0, 0x1d121b17, 0x0b0e090d, 0xadf28bc7, 0xb92db6a8, 0xc8141ea9,
	0x8557f119, 0x4caf7507, 0xbbee99dd, 0xfda37f60, 0x9ff70126, 0xbc5c72f5, 0xc544663b, 0x345bfb7e,
	0x768b4329, 0xdccb23c6, 0x68b6edfc, 0x63b8e4f1, 0xcad731dc, 0x10426385, 0x40139722, 0x2084c611,
	0x7d854a24, 0xf8d2bb3d, 0x11aef932, 0x6dc729a1, 0x4b1d9e2f, 0xf3dcb230, 0xec0d8652, 0xd077c1e3,
	0x6c2bb316, 0x99a970b9, 0xfa119448, 0x2247e964, 0xc4a8fc8c, 0x1aa0f03f, 0xd8567d2c, 0xef223390,
	0xc787494e, 0xc1d938d1, 0xfe8ccaa2, 0x3698d40b, 0xcfa6f581, 0x28a57ade, 0x26dab78e, 0xa43fadbf,
	0xe42c3a9d, 0x0d507892, 0x9b6a5fcc, 0x62547e46, 0xc2f68d13, 0xe890d8b8, 0x5e2e39f7, 0xf582c3af,
	0xbe9f5d80, 0x7c69d093, 0xa96fd52d, 0xb3cf2512, 0x3bc8ac99, 0xa710187d, 0x6ee89c63, 0x7bdb3bbb,
	0x09cd2678, 0xf46e5918, 0x01ec9ab7, 0xa8834f9a, 0x65e6956e, 0x7eaaffe6, 0x0821bccf, 0xe6ef15e8,
	0xd9bae79b, 0xce4a6f36, 0xd4ea9f09, 0xd629b07c, 0xaf31a4b2, 0x312a3f23, 0x30c6a594, 0xc035a266,
	0x37744ebc, 0xa6fc82ca, 0xb0e090d0, 0x1533a7d8, 0x4af10498, 0xf741ecda, 0x0e7fcd50, 0x2f1791f6,
	0x8d764dd6, 0x4d43efb0, 0x54ccaa4d, 0xdfe49604, 0xe39ed1b5, 0x1b4c6a88, 0xb8c12c1f, 0x7f466551,
	0x049d5eea, 0x5d018c35, 0x73fa8774, 0x2efb0b41, 0x5ab3671d, 0x5292dbd2, 0x33e91056, 0x136dd647,
	0x8c9ad761, 0x7a37a10c, 0x8e59f814, 0x89eb133c, 0xeecea927, 0x35b761c9, 0xede11ce5, 0x3c7a47b1,
	0x599cd2df, 0x3f55f273, 0x791814ce, 0xbf73c737, 0xea53f7cd, 0x5b5ffdaa, 0x14df3d6f, 0x867844db,
	0x81caaff3, 0x3eb968c4, 0x2c382434, 0x5fc2a340, 0x72161dc3, 0x0cbce225, 0x8b283c49, 0x41ff0d95,
	0x7139a801, 0xde080cb3, 0x9cd8b4e4, 0x906456c1, 0x617bcb84, 0x70d532b6, 0x74486c5c, 0x42d0b857,
];

const TD2:[u32; 256] = [
	0xa75051f4, 0x65537e41, 0xa4c31a17, 0x5e963a27, 0x6bcb3bab, 0x45f11f9d, 0x58abacfa, 0x03934be3,
	0xfa552030, 0x6df6ad76, 0x769188cc, 0x4c25f502, 0xd7fc4fe5, 0xcbd7c52a, 0x44802635, 0xa38fb562,
	0x5a49deb1, 0x1b6725ba, 0x0e9845ea, 0xc0e15dfe, 0x7502c32f, 0xf012814c, 0x97a38d46, 0xf9c66bd3,
	0x5fe7038f, 0x9c951592, 0x7aebbf6d, 0x59da9552, 0x832dd4be, 0x21d35874, 0x692949e0, 0xc8448ec9,
	0x896a75c2, 0x7978f48e, 0x3e6b9958, 0x71dd27b9, 0x4fb6bee1, 0xad17f088, 0xac66c920, 0x3ab47dce,
	0x4a1863df, 0x3182e51a, 0x33609751, 0x7f456253, 0x77e0b164, 0xae84bb6b, 0xa01cfe81, 0x2b94f908,
	0x68587048, 0xfd198f45, 0x6c8794de, 0xf8b7527b, 0xd323ab73, 0x02e2724b, 0x8f57e31f, 0xab2a6655,
	0x2807b2eb, 0xc2032fb5, 0x7b9a86c5, 0x08a5d337, 0x87f23028, 0xa5b223bf, 0x6aba0203, 0x825ced16,
	0x1c2b8acf, 0xb492a779, 0xf2f0f307, 0xe2a14e69, 0xf4cd65da, 0xbed50605, 0x621fd134, 0xfe8ac4a6,
	0x539d342e, 0x55a0a2f3, 0xe132058a, 0xeb75a4f6, 0xec390b83, 0xefaa4060, 0x9f065e71, 0x1051bd6e,
	0x8af93e21, 0x063d96dd, 0x05aedd3e, 0xbd464de6, 0x8db59154, 0x5d0571c4, 0xd46f0406, 0x15ff6050,
	0xfb241998, 0xe997d6bd, 0x43cc8940, 0x9e7767d9, 0x42bdb0e8, 0x8b880789, 0x5b38e719, 0xeedb79c8,
	0x0a47a17c, 0x0fe97c42, 0x1ec9f884, 0x00000000, 0x86830980, 0xed48322b, 0x70ac1e11, 0x724e6c5a,
	0xfffbfd0e, 0x38560f85, 0xd51e3dae, 0x3927362d, 0xd9640a0f, 0xa621685c, 0x54d19b5b, 0x2e3a2436,
	0x67b10c0a, 0xe70f9357, 0x96d2b4ee, 0x919e1b9b, 0xc54f80c0, 0x20a261dc, 0x4b695a77, 0x1a161c12,
	0xba0ae293, 0x2ae5c0a0, 0xe0433c22, 0x171d121b, 0x0d0b0e09, 0xc7adf28b, 0xa8b92db6, 0xa9c8141e,
	0x198557f1, 0x074caf75, 0xddbbee99, 0x60fda37f, 0x269ff701, 0xf5bc5c72, 0x3bc54466, 0x7e345bfb,
	0x29768b43, 0xc6dccb23, 0xfc68b6ed, 0xf163b8e4, 0xdccad731, 0x85104263, 0x22401397, 0x112084c6,
	0x247d854a, 0x3df8d2bb, 0x3211aef9, 0xa16dc729, 0x2f4b1d9e, 0x30f3dcb2, 0x52ec0d86, 0xe3d077c1,
	0x166c2bb3, 0xb999a970, 0x48fa1194, 0x642247e9, 0x8cc4a8fc, 0x3f1aa0f0, 0x2cd8567d, 0x90ef2233,
	0x4ec78749, 0xd1c1d938, 0xa2fe8cca, 0x0b3698d4, 0x81cfa6f5, 0xde28a57a, 0x8e26dab7, 0xbfa43fad,
	0x9de42c3a, 0x920d5078, 0xcc9b6a5f, 0x4662547e, 0x13c2f68d, 0xb8e890d8, 0xf75e2e39, 0xaff582c3,
	0x80be9f5d, 0x937c69d0, 0x2da96fd5, 0x12b3cf25, 0x993bc8ac, 0x7da71018, 0x636ee89c, 0xbb7bdb3b,
	0x7809cd26, 0x18f46e59, 0xb701ec9a, 0x9aa8834f, 0x6e65e695, 0xe67eaaff, 0xcf0821bc, 0xe8e6ef15,
	0x9bd9bae7, 0x36ce4a6f, 0x09d4ea9f, 0x7cd629b0, 0xb2af31a4, 0x23312a3f, 0x9430c6a5, 0x66c035a2,
	0xbc37744e, 0xcaa6fc82, 0xd0b0e090, 0xd81533a7, 0x984af104, 0xdaf741ec, 0x500e7fcd, 0xf62f1791,
	0xd68d764d, 0xb04d43ef, 0x4d54ccaa, 0x04dfe496, 0xb5e39ed1, 0x881b4c6a, 0x1fb8c12c, 0x517f4665,
	0xea049d5e, 0x355d018c, 0x7473fa87, 0x412efb0b, 0x1d5ab367, 0xd25292db, 0x5633e910, 0x47136dd6,
	0x618c9ad7, 0x0c7a37a1, 0x148e59f8, 0x3c89eb13, 0x27eecea9, 0xc935b761, 0xe5ede11c, 0xb13c7a47,
	0xdf599cd2, 0x733f55f2, 0xce791814, 0x37bf73c7, 0xcdea53f7, 0xaa5b5ffd, 0x6f14df3d, 0xdb867844,
	0xf381caaf, 0xc43eb968, 0x342c3824, 0x405fc2a3, 0xc372161d, 0x250cbce2, 0x498b283c, 0x9541ff0d,
	0x017139a8, 0xb3de080c, 0xe49cd8b4, 0xc1906456, 0x84617bcb, 0xb670d532, 0x5c74486c, 0x5742d0b8,
];

const TD3:[u32; 256] = [
	0xf4a75051, 0x4165537e, 0x17a4c31a, 0x275e963a, 0xab6bcb3b, 0x9d45f11f, 0xfa58abac, 0xe303934b,
	0x30fa5520, 0x766df6ad, 0xcc769188, 0x024c25f5, 0xe5d7fc4f, 0x2acbd7c5, 0x35448026, 0x62a38fb5,
	0xb15a49de, 0xba1b6725, 0xea0e9845, 0xfec0e15d, 0x2f7502c3, 0x4cf01281, 0x4697a38d, 0xd3f9c66b,
	0x8f5fe703, 0x929c9515, 0x6d7aebbf, 0x5259da95, 0xbe832dd4, 0x7421d358, 0xe0692949, 0xc9c8448e,
	0xc2896a75, 0x8e7978f4, 0x583e6b99, 0xb971dd27, 0xe14fb6be, 0x88ad17f0, 0x20ac66c9, 0xce3ab47d,
	0xdf4a1863, 0x1a3182e5, 0x51336097, 0x537f4562, 0x6477e0b1, 0x6bae84bb, 0x81a01cfe, 0x082b94f9,
	0x48685870, 0x45fd198f, 0xde6c8794, 0x7bf8b752, 0x73d323ab, 0x4b02e272, 0x1f8f57e3, 0x55ab2a66,
	0xeb2807b2, 0xb5c2032f, 0xc57b9a86, 0x3708a5d3, 0x2887f230, 0xbfa5b223, 0x036aba02, 0x16825ced,
	0xcf1c2b8a, 0x79b492a7, 0x07f2f0f3, 0x69e2a14e, 0xdaf4cd65, 0x05bed506, 0x34621fd1, 0xa6fe8ac4,
	0x2e539d34, 0xf355a0a2, 0x8ae13205, 0xf6eb75a4, 0x83ec390b, 0x60efaa40, 0x719f065e, 0x6e1051bd,
	0x218af93e, 0xdd063d96, 0x3e05aedd, 0xe6bd464d, 0x548db591, 0xc45d0571, 0x06d46f04, 0x5015ff60,
	0x98fb2419, 0xbde997d6, 0x4043cc89, 0xd99e7767, 0xe842bdb0, 0x898b8807, 0x195b38e7, 0xc8eedb79,
	0x7c0a47a1, 0x420fe97c, 0x841ec9f8, 0x00000000, 0x80868309, 0x2bed4832, 0x1170ac1e, 0x5a724e6c,
	0x0efffbfd, 0x8538560f, 0xaed51e3d, 0x2d392736, 0x0fd9640a, 0x5ca62168, 0x5b54d19b, 0x362e3a24,
	0x0a67b10c, 0x57e70f93, 0xee96d2b4, 0x9b919e1b, 0xc0c54f80, 0xdc20a261, 0x774b695a, 0x121a161c,
	0x93ba0ae2, 0xa02ae5c0, 0x22e0433c, 0x1b171d12, 0x090d0b0e, 0x8bc7adf2, 0xb6a8b92d, 0x1ea9c814,
	0xf1198557, 0x75074caf, 0x99ddbbee, 0x7f60fda3, 0x01269ff7, 0x72f5bc5c, 0x663bc544, 0xfb7e345b,
	0x4329768b, 0x23c6dccb, 0xedfc68b6, 0xe4f163b8, 0x31dccad7, 0x63851042, 0x97224013, 0xc6112084,
	0x4a247d85, 0xbb3df8d2, 0xf93211ae, 0x29a16dc7, 0x9e2f4b1d, 0xb230f3dc, 0x8652ec0d, 0xc1e3d077,
	0xb3166c2b, 0x70b999a9, 0x9448fa11, 0xe9642247, 0xfc8cc4a8, 0xf03f1aa0, 0x7d2cd856, 0x3390ef22,
	0x494ec787, 0x38d1c1d9, 0xcaa2fe8c, 0xd40b3698, 0xf581cfa6, 0x7ade28a5, 0xb78e26da, 0xadbfa43f,
	0x3a9de42c, 0x78920d50, 0x5fcc9b6a, 0x7e466254, 0x8d13c2f6, 0xd8b8e890, 0x39f75e2e, 0xc3aff582,
	0x5d80be9f, 0xd0937c69, 0xd52da96f, 0x2512b3cf, 0xac993bc8, 0x187da710, 0x9c636ee8, 0x3bbb7bdb,
	0x267809cd, 0x5918f46e, 0x9ab701ec, 0x4f9aa883, 0x956e65e6, 0xffe67eaa, 0xbccf0821, 0x15e8e6ef,
	0xe79bd9ba, 0x6f36ce4a, 0x9f09d4ea, 0xb07cd629, 0xa4b2af31, 0x3f23312a, 0xa59430c6, 0xa266c035,
	0x4ebc3774, 0x82caa6fc, 0x90d0b0e0, 0xa7d81533, 0x04984af1, 0xecdaf741, 0xcd500e7f, 0x91f62f17,
	0x4dd68d76, 0xefb04d43, 0xaa4d54cc, 0x9604dfe4, 0xd1b5e39e, 0x6a881b4c, 0x2c1fb8c1, 0x65517f46,
	0x5eea049d, 0x8c355d01, 0x877473fa, 0x0b412efb, 0x671d5ab3, 0xdbd25292, 0x105633e9, 0xd647136d,
	0xd7618c9a, 0xa10c7a37, 0xf8148e59, 0x133c89eb, 0xa927eece, 0x61c935b7, 0x1ce5ede1, 0x47b13c7a,
	0xd2df599c, 0xf2733f55, 0x14ce7918, 0xc737bf73, 0xf7cdea53, 0xfdaa5b5f, 0x3d6f14df, 0x44db8678,
	0xaff381ca, 0x68c43eb9, 0x24342c38, 0xa3405fc2, 0x1dc37216, 0xe2250cbc, 0x3c498b28, 0x0d9541ff,
	0xa8017139, 0x0cb3de08, 0xb4e49cd8, 0x56c19064, 0xcb84617b, 0x32b670d5, 0x6c5c7448, 0xb85742d0,
];

// The AES block size in bytes.
const BlockSize: usize = 16;

fn uint32(b: &[u8]) -> u32 { // BigEndian
    assert!(b.len() >= 4); // bounds check
    (b[3] as u32) | (b[2] as u32) << 8 | (b[1] as u32) << 16 | (b[0] as u32) << 24
}

//fn uint32(b: &[u8; 4]) -> u32 {
    //u32::from(b[3]) | (u32::from(b[2]) << 8) | (u32::from(b[1]) << 16) | (u32::from(b[0]) << 24)
//}

//fn put_uint32(b: &mut [u8; 4], v: u32) {
fn put_uint32(b: &mut [u8], v: u32) {
    b[0] = (v >> 24) as u8;
    b[1] = (v >> 16) as u8;
    b[2] = (v >> 8) as u8;
    b[3] = v as u8;
}

//fn uint64(b: &[u8; 8]) -> u64 {
fn uint64(b: &[u8]) -> u64 {
    u64::from(b[7]) | (u64::from(b[6]) << 8) | (u64::from(b[5]) << 16) | (u64::from(b[4]) << 24) |
    (u64::from(b[3]) << 32) | (u64::from(b[2]) << 40) | (u64::from(b[1]) << 48) | (u64::from(b[0]) << 56)
}

// fn put_uint64(b: &mut [u8; 8], v: u64) {
fn put_uint64(b: &mut [u8], v: u64) {
    b[0] = (v >> 56) as u8;
    b[1] = (v >> 48) as u8;
    b[2] = (v >> 40) as u8;
    b[3] = (v >> 32) as u8;
    b[4] = (v >> 24) as u8;
    b[5] = (v >> 16) as u8;
    b[6] = (v >> 8) as u8;
    b[7] = v as u8;
}

// XORBytes устанавливает dst[i] = x[i] ^ y[i] для всех i < n = min(len(x), len(y)),
// возвращая n, количество байтов, записанных в dst.
fn xor_bytes(dst: &mut [u8], x: &[u8], y: &[u8]) -> usize {
    let n = x.len().min(y.len());
    if n == 0 {
        return 0;
    }
    if n > dst.len() {
        panic!("XORBytes: dst too short");
    }

    for i in 0..n {
        dst[i] = x[i] ^ y[i];
    }

    n
}

// Apply sbox0 to each byte in w.
fn subw(w: u32) -> u32 {
    (SBOX0[(w >> 24) as usize] as u32) << 24 |
    (SBOX0[((w >> 16) & 0xff) as usize] as u32) << 16 |
    (SBOX0[((w >> 8) & 0xff) as usize] as u32) << 8 |
    (SBOX0[(w & 0xff) as usize] as u32)
}

// Rotate
fn rotw(w: u32) -> u32 {
    (w << 8) | (w >> 24)
}

// Key expansion algorithm.
// fn expand_key(key: &[u8;16], enc: &mut Vec<u32>, dec: &mut Vec<u32>) {
fn expand_key(key: &[u8;16], enc: &mut [u32;44], dec: &mut [u32;44]) {
    let nk = key.len() / 4;

    // Encryption key setup.
    for i in 0..nk {
        enc[i] = uint32(&key[4 * i..]);//enc[i] = uint32l(&key[4 * i..]);//enc.push(uint32(&key[4 * i..]));
    }

    let len_enc = enc.len();
    for i in nk..len_enc {
        let mut t = enc[i - 1];
        if i % nk == 0 {
            t = subw(rotw(t)) ^ ((POWX[i / nk - 1] as u32) << 24);
        } else if nk > 6 && i % nk == 4 {
            t = subw(t);
        }
        enc[i] = enc[i - nk] ^ t; // enc.push(enc[i - nk] ^ t);
    }

    // Derive decryption key from encryption key, if dec is not empty.
	// Reverse the 4-word round key sets from enc to produce dec.
	// All sets but the first and last get the MixColumn transform applied.
    //if !dec.is_empty() {
        let n = enc.len();
        for i in (0..n).step_by(4) {
            let ei = n - i - 4;
            for j in 0..4 {
                let mut x = enc[ei + j];
                if i > 0 && i + 4 < n {
                    x = TD0[SBOX0[(x >> 24) as usize] as usize] ^
                         TD1[SBOX0[((x >> 16) & 0xff) as usize] as usize] ^
                         TD2[SBOX0[((x >> 8) & 0xff) as usize] as usize] ^
                         TD3[SBOX0[(x & 0xff) as usize] as usize];
                }
                dec[i+j] = x; //dec.push(x);
            }
        }
    //}
}

// Encrypt one block from src into dst, using the expanded key xk.
fn encrypt_block(xk: &[u32;44], dst: &mut [u8;16], src: &[u8;16]) {
    //assert!(src.len() >= 16); // early bounds check
    let s0 = uint32(&src[0..4]);
    let s1 = uint32(&src[4..8]);
    let s2 = uint32(&src[8..12]);
    let s3 = uint32(&src[12..16]);


    // First round just XORs input with key.
    let mut s0 = s0 ^ xk[0];
    let mut s1 = s1 ^ xk[1];
    let mut s2 = s2 ^ xk[2];
    let mut s3 = s3 ^ xk[3];

    // Middle rounds shuffle using tables.
    let nr = xk.len() / 4 - 2; // - 2: one above, one more below
    let mut k = 4;
    let mut t0= 0u32;
    let mut t1= 0u32;
    let mut t2= 0u32;
    let mut t3= 0u32;

    for _ in 0..nr {
        t0 = xk[k]
            ^ TE0[(s0 >> 24) as u8 as usize]
            ^ TE1[(s1 >> 16) as u8 as usize]
            ^ TE2[(s2 >> 8) as u8 as usize]
            ^ TE3[(s3 & 0xff) as u8 as usize];
        t1 = xk[k + 1]
            ^ TE0[(s1 >> 24) as u8 as usize]
            ^ TE1[(s2 >> 16) as u8 as usize]
            ^ TE2[(s3 >> 8) as u8 as usize]
            ^ TE3[(s0 & 0xff) as u8 as usize];
        t2 = xk[k + 2]
            ^ TE0[(s2 >> 24) as u8 as usize]
            ^ TE1[(s3 >> 16) as u8 as usize]
            ^ TE2[(s0 >> 8) as u8 as usize]
            ^ TE3[(s1 & 0xff) as u8 as usize];
        t3 = xk[k + 3]
            ^ TE0[(s3 >> 24) as u8 as usize]
            ^ TE1[(s0 >> 16) as u8 as usize]
            ^ TE2[(s1 >> 8) as u8 as usize]
            ^ TE3[(s2 & 0xff) as u8 as usize];
        k += 4;
        s0 = t0;
        s1 = t1;
        s2 = t2;
        s3 = t3;
    }

    // Last round uses s-box directly and XORs to produce output.
    s0 = ((SBOX0[(t0 >> 24) as usize] as u32) << 24)
        | ((SBOX0[((t1 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t2 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t3 & 0xff) as usize] as u32);

    s1 = ((SBOX0[(t1 >> 24) as usize] as u32) << 24)
        | ((SBOX0[((t2 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t3 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t0 & 0xff) as usize] as u32);

    s2 = ((SBOX0[(t2 >> 24) as usize] as u32) << 24)
        | ((SBOX0[((t3 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t0 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t1 & 0xff) as usize] as u32);

    s3 = ((SBOX0[(t3 >> 24) as usize] as u32) << 24)
        | ((SBOX0[((t0 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t1 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t2 & 0xff) as usize] as u32);

    s0 ^= xk[k];
    s1 ^= xk[k + 1];
    s2 ^= xk[k + 2];
    s3 ^= xk[k + 3];

    //assert!(dst.len() >= 16); // early bounds check
    put_uint32(&mut dst[0..4], s0);
    put_uint32(&mut dst[4..8], s1);
    put_uint32(&mut dst[8..12], s2);
    put_uint32(&mut dst[12..16], s3);
}

// Decrypt one block from src into dst, using the expanded key xk.
fn decrypt_block(xk: &[u32;44], dst: &mut [u8;16], src: &[u8;16]) {
    assert!(src.len() >= 16); // early bounds check
    let mut s0 = uint32(&src[0..4]);
    let mut s1 = uint32(&src[4..8]);
    let mut s2 = uint32(&src[8..12]);
    let mut s3 = uint32(&src[12..16]);

    // First round just XORs input with key.
    s0 ^= xk[0];
    s1 ^= xk[1];
    s2 ^= xk[2];
    s3 ^= xk[3];

    // Middle rounds shuffle using tables.
	// Number of rounds is set by length of expanded key.
    let nr = xk.len() / 4 - 2; // - 2: one above, one more below
    let mut k = 4;
    let mut t0= 0u32;
    let mut t1= 0u32;
    let mut t2= 0u32;
    let mut t3= 0u32;

    for _ in 0..nr {
        t0 = xk[k]
            ^ TD0[(s0 >> 24) as u8 as usize]
            ^ TD1[(s3 >> 16) as u8 as usize]
            ^ TD2[(s2 >> 8) as u8 as usize]
            ^ TD3[(s1 & 0xff) as u8 as usize];
        t1 = xk[k + 1]
            ^ TD0[(s1 >> 24) as u8 as usize]
            ^ TD1[(s0 >> 16) as u8 as usize]
            ^ TD2[(s3 >> 8) as u8 as usize]
            ^ TD3[(s2 & 0xff) as u8 as usize];
        t2 = xk[k + 2]
            ^ TD0[(s2 >> 24) as u8 as usize]
            ^ TD1[(s1 >> 16) as u8 as usize]
            ^ TD2[(s0 >> 8) as u8 as usize]
            ^ TD3[(s3 & 0xff) as u8 as usize];
        t3 = xk[k + 3]
            ^ TD0[(s3 >> 24) as u8 as usize]
            ^ TD1[(s2 >> 16) as u8 as usize]
            ^ TD2[(s1 >> 8) as u8 as usize]
            ^ TD3[(s0 & 0xff) as u8 as usize];
        k += 4;
        s0 = t0;
        s1 = t1;
        s2 = t2;
        s3 = t3;
    }

    // Last round uses s-box directly and XORs to produce output.
    s0 = (SBOX1[(t0 >> 24) as usize] as u32) << 24
        | (SBOX1[((t3 >> 16) & 0xff) as usize] as u32) << 16
        | (SBOX1[((t2 >> 8) & 0xff) as usize] as u32) << 8
        | (SBOX1[(t1 & 0xff) as usize] as u32);

    s1 = (SBOX1[(t1 >> 24) as usize] as u32) << 24
        | (SBOX1[((t0 >> 16) & 0xff) as usize] as u32) << 16
        | (SBOX1[((t3 >> 8) & 0xff) as usize] as u32) << 8
        | (SBOX1[(t2 & 0xff) as usize] as u32);

    s2 = (SBOX1[(t2 >> 24) as usize] as u32) << 24
        | (SBOX1[((t1 >> 16) & 0xff) as usize] as u32) << 16
        | (SBOX1[((t0 >> 8) & 0xff) as usize] as u32) << 8
        | (SBOX1[(t3 & 0xff) as usize] as u32);

    s3 = (SBOX1[(t3 >> 24) as usize] as u32) << 24
        | (SBOX1[((t2 >> 16) & 0xff) as usize] as u32) << 16
        | (SBOX1[((t1 >> 8) & 0xff) as usize] as u32) << 8
        | (SBOX1[(t0 & 0xff) as usize] as u32);

    s0 ^= xk[k];
    s1 ^= xk[k + 1];
    s2 ^= xk[k + 2];
    s3 ^= xk[k + 3];

    put_uint32(&mut dst[0..4], s0);
    put_uint32(&mut dst[4..8], s1);
    put_uint32(&mut dst[8..12], s2);
    put_uint32(&mut dst[12..16], s3);
}

// A cipher is an instance of AES encryption using a particular key.
pub struct Aes256Cipher {
    enc: [u32; 44], // enc []uint32
    dec: [u32; 44], // dec []uint32
}

impl Aes256Cipher {
    //
    pub fn new(key: &[u8;16]) -> Aes256Cipher {

		println!("the initial key is : {:?}", &key);
		let mut enc = [0u32; 44];
		let mut dec = [0u32; 44];
		expand_key(key, &mut enc, &mut dec);
		//println!("enc key is : {:?}", &enc);
		//println!("dec key is : {:?}", &dec);

        return Aes256Cipher {
            enc,
            dec,
        };
    }

    pub fn block_size(&self) -> usize {
        return BlockSize;
    }

	pub fn encrypt(&self, dst: &mut [u8;16], src: &[u8;16]) {
		encrypt_block(&self.enc, dst, &src);
	}

	pub fn decrypt(&self, dst: &mut [u8;16], src: &[u8;16]) {
		decrypt_block(&self.dec, dst, &src)
	}
}

// NewCipher creates and returns a new [cipher.Block].
// The key argument should be the AES key,
// either 16, 24, or 32 bytes to select
// AES-128, AES-192, or AES-256.
pub fn new_cipher(key: &[u8;16]) -> Aes256Cipher {
    //let n = 16 + 28; // let n = key.len() + 28;

    let c = Aes256Cipher::new(key);
    //let rounds = 10;
    //expand_key(key, c.enc, c.dec);
    c
}

//==============================================================================
// gcmFieldElement represents a value in GF(2¹²⁸). In order to reflect the GCM
// standard and make binary.BigEndian suitable for marshaling these values, the
// bits are stored in big endian order. For example:
//
//	the coefficient of x⁰ can be obtained by v.low >> 63.
//	the coefficient of x⁶³ can be obtained by v.low & 1.
//	the coefficient of x⁶⁴ can be obtained by v.high >> 63.
//	the coefficient of x¹²⁷ can be obtained by v.high & 1.
#[derive(Debug, Copy, Clone, Default)]
struct GcmFieldElement {
    low: u64,
    high: u64,
}

const GCM_REDUCTION_TABLE: [u16; 16] = [
    0x0000, 0x1c20, 0x3840, 0x2460,
    0x7080, 0x6ca0, 0x48c0, 0x54e0,
    0xe100, 0xfd20, 0xd940, 0xc560,
    0x9180, 0x8da0, 0xa9c0, 0xb5e0,
];

// Обратная работа с битами - изменение порядка битов 4-битного числа.
fn reverse_bits(i: usize) -> usize {
    let mut i = i;
    i = ((i << 2) & 0xc) | ((i >> 2) & 0x3);
    i = ((i << 1) & 0xa) | ((i >> 1) & 0x5);
    i
}

// gcm_add складывает два элемента GF(2¹²⁸) и возвращает сумму.
fn gcm_add(x: &GcmFieldElement, y: &GcmFieldElement) -> GcmFieldElement {
    // Сложение в поле характеристики 2 — это просто XOR.
    GcmFieldElement {
        low: x.low ^ y.low,
        high: x.high ^ y.high,
    }
}

// gcm_double возвращает результат удвоения элемента GF(2¹²⁸).
fn gcm_double(x: &GcmFieldElement) -> GcmFieldElement {
    let msb_set = x.high & 1 == 1;

    // Из-за порядка бит удвоение фактически является правым сдвигом.
    let mut double = GcmFieldElement {
        high: x.high >> 1,
        low: x.low >> 1,
    };
    double.high |= x.low << 63;

    // Если старший бит был установлен перед сдвигом, он
    // становится термном x^128. Это больше, чем
    // неприводимый многочлен, поэтому результат должен быть сокращен.
    // Неприводимый многочлен: 1+x+x^2+x^7+x^128. Мы можем вычесть это, чтобы
    // устранить термин x^128, что также означает вычитание других
    // четырех терминов. В полях характеристики 2, вычитание == сложению ==
    // XOR.
    if msb_set {
        double.low ^= 0xe100000000000000;
    }

    double
}


// Gcm represents a Galois Counter Mode with a specific key.
pub struct Gcm {
	cipher: Aes256Cipher,
	nonce_size: usize,
	tag_size: usize,
	// product_table содержит первые шестнадцать степеней ключа H.
	// Однако они находятся в порядке побитового реверса.
	product_table: [GcmFieldElement; 16],
}

const GCM_BLOCK_SIZE: usize = 16;
const GCM_TAG_SIZE: usize = 16;
const GCM_MINIMUM_TAG_SIZE: usize = 12; // NIST SP 800-38D рекомендует теги размером 12 байт и больше.
const GCM_STANDARD_NONCE_SIZE: usize = 12;


impl Gcm {
	//
	pub fn new_gcm_with_nonce_and_tag_size(cipher: Aes256Cipher, nonce_size: usize, tag_size: usize) -> Gcm {
        if tag_size < GCM_MINIMUM_TAG_SIZE || tag_size > GCM_BLOCK_SIZE {
            //return Err(Box::new(GcmError {
                //details: String::from("Incorrect tag size given to GCM"),
            //}));
			panic!("Incorrect tag size given to GCM");
        }

        if nonce_size <= 0 {
            //return Err(Box::new(GcmError {
                //details: String::from("The nonce can't have zero length, or the security of the key will be immediately compromised"),
            //}));
			panic!("The nonce can't have zero length, or the security of the key will be immediately compromised");
        }

        //if let Some(gcm_able) = cipher.as_gcm() {
            //return gcm_able.new_gcm(nonce_size, tag_size);
        //}

        if cipher.block_size() != GCM_BLOCK_SIZE {
            //return Err(Box::new(GcmError {
                //details: String::from("NewGCM requires 128-bit block cipher"),
            //}));
			panic!("NewGCM requires 128-bit block cipher");
        }

		let mut zero = [0u8; GCM_BLOCK_SIZE];
        let mut key = [0u8; GCM_BLOCK_SIZE];
		println!("key is : {:?}", &key);
        cipher.encrypt(&mut key, &zero);
		println!("after cipher.Encrypt key is : {:?}", &key);

        let mut g = Gcm {
            cipher,
            nonce_size,
            tag_size,
            product_table: Default::default(),
        };

        let mut x = GcmFieldElement {
            low: u64::from_be_bytes(key[0..8].try_into().unwrap()),
			high: u64::from_be_bytes(key[8..16].try_into().unwrap())
        };
        g.product_table[reverse_bits(1)] = x;

        for i in (2..16).step_by(2) {
            g.product_table[reverse_bits(i)] = gcm_double(&g.product_table[reverse_bits(i / 2)]);
            g.product_table[reverse_bits(i + 1)] = gcm_add(&g.product_table[reverse_bits(i)], &x);
        }

        //Ok(g)
		g
    }

	// nonce_size returns the size of the nonce that must be passed to Seal
	// and Open.
	pub fn nonce_size() -> usize{
		return GCM_STANDARD_NONCE_SIZE;
	}

	// Overhead returns the maximum difference between the lengths of a
	// plaintext and its ciphertext.
	pub fn overhead() -> usize{
		return GCM_TAG_SIZE;
	}

	pub fn seal(&self, dst: &[u8], nonce: &[u8], plaintext: &[u8], data: &[u8]) -> Vec<u8> {
        if nonce.len() != self.nonce_size {
            panic!("crypto/cipher: incorrect nonce length given to GCM");
        }
        if plaintext.len() as u64 > ((1 << 32) - 2) * self.cipher.block_size() as u64 {
            panic!("crypto/cipher: message too large for GCM");
        }

        let mut ret = dst.to_vec(); // В Rust мы создаем новый вектор
        ret.resize(plaintext.len() + self.tag_size, 0);

        // Программа проверяет пересечение буферов - здесь будет потребоваться реализация
        // let out = &mut ret; // Пример, как можно использовать `ret`

        let mut counter = [0u8; GCM_BLOCK_SIZE];
        let mut tag_mask = [0u8; GCM_BLOCK_SIZE];

        self.derive_counter(&mut counter, nonce);

        self.cipher.encrypt(&mut tag_mask, &counter);
        gcm_inc32(&mut counter);

        self.counter_crypt(&mut ret[0..plaintext.len() + self.tag_size], plaintext, &mut counter);

        let mut tag = [0u8; GCM_TAG_SIZE];
        self.auth(&mut tag, &ret[0..plaintext.len()], data, &tag_mask);
        ret[plaintext.len()..].copy_from_slice(&tag);

        ret
    }

	// fn open(&self, dst: &[u8], nonce: &[u8], ciphertext: &[u8], data: &[u8]) -> Result<Vec<u8>, &'static str> {
	pub fn open(&self, dst: &[u8], nonce: &[u8], ciphertext: &[u8], data: &[u8]) -> Vec<u8> {
        if nonce.len() != self.nonce_size {
            panic!("crypto/cipher: incorrect nonce length given to GCM");
        }

        // Проверка на корректность размера тега
        if self.tag_size < GCM_MINIMUM_TAG_SIZE {
            panic!("crypto/cipher: incorrect GCM tag size");
        }

        if ciphertext.len() < self.tag_size {
            panic!("cipher: message authentification failed 1");//return Err(ERR_OPEN);
        }

        if ciphertext.len() as u64 > ((1 << 32) - 2) * self.cipher.block_size() as u64 + self.tag_size as u64 {
            panic!("cipher: message authentification failed 2");//return Err(ERR_OPEN);
        }

        let tag = &ciphertext[ciphertext.len() - self.tag_size..];
        let ciphertext = &ciphertext[..ciphertext.len() - self.tag_size];

        let mut counter = [0u8; GCM_BLOCK_SIZE];
        let mut tag_mask = [0u8; GCM_BLOCK_SIZE];

        self.derive_counter(&mut counter, nonce);

        self.cipher.encrypt(&mut tag_mask, &counter);
        gcm_inc32(&mut counter);

        let mut expected_tag = [0u8; GCM_TAG_SIZE];
        self.auth(&mut expected_tag, ciphertext, data, &tag_mask);

        let mut ret = Vec::with_capacity(ciphertext.len());
        ret.resize(ciphertext.len(), 0);

        // Проверка пересечения буферов
        // Let me know if you need to implement specific buffer overlap checks.

        if constant_time_compare(&expected_tag[..self.tag_size], tag) != 1 {
            // Обнуляем выходной буфер в случае несовпадения тега
            for byte in ret.iter_mut() {
                *byte = 0;
            }
            panic!("cipher: message authentification failed 3");//return Err(ERR_OPEN);
        }

        self.counter_crypt(&mut ret, ciphertext, &mut counter);

        ret//Ok(ret)
    }

	fn update_blocks(&self, y: &mut GcmFieldElement, blocks: &[u8]) {
        let mut remaining_blocks = blocks;

        while !remaining_blocks.is_empty() {
            y.low ^= uint64(&remaining_blocks[0..8]); // y.low ^= uint64(&remaining_blocks[0..8].try_into().unwrap());
            y.high ^= uint64(&remaining_blocks[8..16]); // y.high ^= uint64(&remaining_blocks[8..16].try_into().unwrap());
            self.mul(y);
            remaining_blocks = &remaining_blocks[GCM_BLOCK_SIZE..];
        }
    }

    // update расширяет y еще полиномными терминами из данных. Если данные не являются
    // кратными gcm_block_size байт, то остаток заполняется нулями.
    fn update(&self, y: &mut GcmFieldElement, data: &[u8]) {
        let full_blocks = (data.len() >> 4) << 4;
        self.update_blocks(y, &data[0..full_blocks]);

        if data.len() != full_blocks {
            let mut partial_block = [0u8; GCM_BLOCK_SIZE];
            partial_block[0..data.len() - full_blocks].copy_from_slice(&data[full_blocks..]);
            self.update_blocks(y, &partial_block);
        }
    }



    // counterCrypt шифрует в out, используя g.cipher в режиме счётчика.
    fn counter_crypt(&self, out: &mut [u8], in_data: &[u8], counter: &mut [u8; GCM_BLOCK_SIZE]) {
        let mut mask = [0u8; GCM_BLOCK_SIZE];

        let mut remaining_in = in_data;

		let mut start_index = 0;
        while remaining_in.len() >= GCM_BLOCK_SIZE {
            self.cipher.encrypt(&mut mask, counter);
            gcm_inc32(counter);
			//let start_index = out.len() - remaining_in.len();
            xor_bytes(&mut out[start_index..], &remaining_in[..GCM_BLOCK_SIZE], &mask);
            remaining_in = &remaining_in[GCM_BLOCK_SIZE..];
			start_index+=GCM_BLOCK_SIZE;
        }

        if !remaining_in.is_empty() {
            self.cipher.encrypt(&mut mask, counter);
            gcm_inc32(counter);
			//let start_index = out.len() - remaining_in.len();
            xor_bytes(&mut out[start_index..], remaining_in, &mask);
        }
    }

	// mul устанавливает y равным y*H, где H - ключ GCM, фиксированный во время NewGCMWithNonceSize.
	fn mul(&self, y: &mut GcmFieldElement) {
        let mut z = GcmFieldElement { low: 0, high: 0 };

        for i in 0..2 {
            let mut word = if i == 1 { y.low } else { y.high };

            // Умножение выполняется путем умножения z на 16 и добавления
            // одного из предвычисленных кратных H.
            for _ in (0..64).step_by(4) {
                let msw = (z.high & 0xf) as usize;
                z.high >>= 4;
                z.high |= z.low << 60;
                z.low >>= 4;
                z.low ^= (GCM_REDUCTION_TABLE[msw] as u64) << 48;

                // Значения в |table| упорядочены по
                // порядку бит для little-endian. Смотрите комментарий
                // в NewGCMWithNonceSize.
                let t = &self.product_table[(word & 0xf) as usize];

                z.low ^= t.low;
                z.high ^= t.high;
                word >>= 4;
            }
        }

        *y = z;
    }

	// derive_counter вычисляет начальное состояние счетчика GCM из данного нонса.
	fn derive_counter(&self, counter: &mut [u8; GCM_BLOCK_SIZE], nonce: &[u8]) {
        if nonce.len() == GCM_STANDARD_NONCE_SIZE {
            counter[..nonce.len()].copy_from_slice(nonce);
            counter[GCM_BLOCK_SIZE - 1] = 1;
        } else {
            let mut y = GcmFieldElement { low: 0, high: 0 };
            self.update(&mut y, nonce);
            y.high ^= (nonce.len() as u64) * 8;
            self.mul(&mut y);
            put_uint64(&mut counter[..8], y.low); // put_uint64(&mut counter[..8].try_into().unwrap(), y.low);
            put_uint64(&mut counter[8..], y.high); // put_uint64(&mut counter[8..].try_into().unwrap(), y.high);
        }
    }

	// auth вычисляет GHASH(ciphertext, additionalData), маскирует результат с
    // tagMask и записывает результат в out.
    fn auth(&self, out: &mut [u8; GCM_TAG_SIZE], ciphertext: &[u8], additional_data: &[u8], tag_mask: &[u8; GCM_TAG_SIZE]) {
        let mut y = GcmFieldElement { low: 0, high: 0 };
        self.update(&mut y, additional_data);
        self.update(&mut y, ciphertext);

        y.low ^= (additional_data.len() as u64) * 8;
        y.high ^= (ciphertext.len() as u64) * 8;

        self.mul(&mut y);

        put_uint64(&mut out[..8], y.low); // put_uint64(&mut out[..8].try_into().unwrap(), y.low);
        put_uint64(&mut out[8..], y.high); // put_uint64(&mut out[8..].try_into().unwrap(), y.high);

		let out_clone = out.clone();
        xor_bytes(out, &out_clone, tag_mask);
    }
}

// gcm_inc32 рассматривает последние четыре байта counterBlock как значение в big-endian
// и увеличивает его.
fn gcm_inc32(counter_block: &mut [u8; GCM_BLOCK_SIZE]) {
	let start_index = counter_block.len() - 4;
	let ctr = &mut counter_block[start_index..];
	let value = uint32(ctr) + 1; //  let value = uint32(ctr.try_into().unwrap()) + 1;
	put_uint32(ctr, value); // put_uint32(ctr.try_into().unwrap(), value);
}

pub fn constant_time_compare(x: &[u8], y: &[u8]) -> i32 {
    if x.len() != y.len() {
        return 0;
    }

    let mut v: u8 = 0;

    for (a, b) in x.iter().zip(y.iter()) {
        v |= a ^ b;
    }

    //constant_time_byte_eq(v, 0)
	if v==0u8 {1} else {0}
}

// ConstantTimeByteEq returns 1 if x == y and 0 otherwise.
//pub fn constant_time_byte_eq(x: u8, y: u8) -> i32 {
    //((x ^ y) as u32).wrapping_sub(1) >> 31 as i32
//}

// NewGCM returns the given 128-bit, block cipher wrapped in Galois Counter Mode
// with the standard nonce length.
//
// In general, the GHASH operation performed by this implementation of GCM is not constant-time.
// An exception is when the underlying [Block] was created by aes.NewCipher
// on systems with hardware support for AES. See the [crypto/aes] package documentation for details.
pub fn new_gcm(cipher: Aes256Cipher) -> Gcm {
	Gcm::new_gcm_with_nonce_and_tag_size(cipher, GCM_STANDARD_NONCE_SIZE, GCM_TAG_SIZE)
}
/*
#[test]
fn test_decrypt_with_go_data_1(){
    //let wrapper =
    let key:[u8;16] = [16, 155, 224, 230, 56, 239, 31, 60, 227, 196, 115, 225, 25, 197, 2, 193];
    println!("the initial key is : {:?}", &key);
    let block = new_cipher(&key);
    // enc key [3873479440 1008725816 3782460643 3238184217 848882871 243729295 4025796460 787874421 2947133122 2704190797 1322936865 1613523028 2407128963 777482958 1619859695 10580155 1696263642 1263378196 734038011 727840576 1827340962 665230774 208017997 654830861 3139946455 2625722977 2431074348 3084954913 1183720962 3658447971 1256733775 4245197166 3654948355 64318048 1228756527 3023423297 1515684062 1501751998 281001105 2760539088 706674377 1939608695 1663281382 3349795638]
    // dec key [706674377 1939608695 1663281382 3349795638 4053962125 3980680061 1908195981 3697545775 3241353926 484890352 2633538032 2916667554 2563652481 3721786422 2149469952 824199506 141129626 1159450551 1573632822 2973632082 1288006850 1299363885 416314497 3975527780 687288487 28824815 1436732588 4096117221 1103519347 692118600 1410668611 2709980489 1711649722 1753659963 2102787083 4119993610 1314089349 243468673 366167600 2294800641 3873479440 1008725816 3782460643 3238184217]
    let aes_gcm = new_gcm(block);

    let additional = [23, 3, 3, 16, 252];//let additional = &wrapper[0..5];
    //let ciphertext = &wrapper[5..];
    let ciphertext = [243, 162, 154, 105, 78, 213, 197, 146, 117, 237, 140, 236, 107, 95, 248, 64, 247, 16, 195, 191, 152, 141, 70, 248, 1, 70, 210, 187, 224, 147, 81, 89,
        88, 183, 27, 62, 92, 247, 40, 32, 187, 181, 63, 9, 177, 145, 3, 169, 59, 173, 132, 51, 148, 209, 239, 5, 70, 103, 121, 58, 15, 185, 126, 116, 54, 62, 94, 49, 222, 8, 174, 78, 123, 9, 67, 117, 218, 181, 173, 95, 48, 26, 186, 249, 48, 117, 72, 49, 173, 184, 35, 100, 101, 22, 15, 12, 76, 147, 145, 78, 77, 122, 122, 12, 56, 156, 128, 193, 83, 6, 161, 216, 185, 100, 35, 106, 120, 86, 171, 61, 255, 188, 223, 9, 7, 132, 139, 180, 192, 89, 222, 149, 60, 185, 164, 150, 228, 11, 159, 17, 165, 154, 90, 180, 14, 186, 199, 38, 213, 23, 32, 252, 107, 49, 148, 177, 206, 123, 209, 171, 183, 201, 142, 178, 239, 201, 137, 211, 139, 86, 234, 2, 135, 38, 241, 36, 175, 206, 225, 42, 191, 64, 9, 118, 223, 88, 248, 181, 48, 47, 96, 61, 18, 246, 225, 80, 35, 71, 41, 250, 233, 228, 199, 184, 182, 20, 187, 244, 102, 85, 67, 145, 133, 182, 97, 4, 1, 125, 13, 145, 90, 185, 104, 84, 144, 81, 197, 53, 142, 56, 129, 159, 81, 199, 95, 116, 39, 91, 226, 24, 177, 10, 254, 40, 30, 180, 33, 4, 58, 234, 60, 173, 210, 69, 177, 194, 171, 162, 133, 55, 13, 196, 13, 69, 32, 14, 235, 119, 71, 193, 194, 121, 133, 11, 207, 70, 9, 55, 222, 71, 167, 216, 160, 72, 208, 190, 193, 170, 179, 15, 140, 220, 231, 248, 234, 186, 100, 21, 55, 189, 95, 95, 255, 45, 221, 29, 79, 227, 231, 204, 247, 193, 111, 107, 34, 210, 129, 186, 252, 159, 95, 34, 247, 83, 163, 91, 225, 128, 28, 162, 119, 10, 214, 239, 20, 86, 134, 90, 223, 60, 110, 158, 254, 198, 87, 218, 250, 236, 104, 206, 90, 179, 111, 231, 106, 120, 15, 164, 182, 207, 94, 37, 130, 33, 5, 218, 154, 66, 103, 12, 32, 81, 47, 50, 5, 209, 124, 53, 183, 35, 10, 172, 40, 224, 224, 121, 106, 230, 240, 255, 179, 2, 205, 112, 18, 68, 162, 5, 178, 74, 59, 64, 203, 37, 190, 148, 84, 87, 171, 113, 149, 91, 40, 7, 123, 52, 45, 254, 173, 25, 223, 243, 20, 25, 249, 130, 92, 5, 153, 150, 54, 14, 37, 82, 37, 220, 33, 144, 145, 117, 78, 225, 115, 112, 208, 68, 97, 136, 87, 177, 124, 24, 49, 36, 155, 247, 48, 203, 167, 39, 13, 28, 158, 159, 106, 206, 61, 11, 160, 16, 156, 53, 85, 69, 214, 18, 22, 162, 113, 141, 178, 210, 21, 131, 119, 197, 53, 82, 170, 85, 128, 47, 186, 108, 89, 129, 130, 193, 170, 134, 140, 23, 40, 229, 80, 101, 109, 67, 27, 43, 255, 41, 16, 6, 131, 13, 123, 143, 141, 7, 42, 21, 179, 167, 144, 201, 182, 46, 37, 127, 215, 55, 211, 202, 110, 176, 28, 30, 143, 74, 120, 4, 207, 125, 155, 216, 193, 166, 76, 127, 103, 208, 61, 127, 81, 10, 70, 195, 105, 210, 42, 143, 134, 111, 88, 221, 183, 140, 132, 179, 88, 188, 4, 184, 194, 124, 105, 9, 236, 191, 60, 190, 61, 151, 145, 98, 25, 54, 67, 92, 45, 232, 102, 166, 60, 39, 68, 233, 76, 247, 20, 162, 150, 169, 133, 100, 209, 137, 237, 201, 247, 3, 227, 102, 128, 88, 203, 178, 0, 105, 92, 226, 129, 183, 83, 111, 187, 220, 74, 111, 146, 223, 241, 35, 219, 211, 39, 115, 242, 49, 178, 234, 74, 180, 21, 105, 22, 183, 212, 44, 210, 110, 195, 185, 87, 249, 87, 169, 165, 69, 135, 48, 215, 28, 79, 166, 41, 50, 84, 131, 48, 72, 143, 147, 207, 48, 182, 81, 249, 166, 236, 168, 185, 22, 173, 204, 130, 118, 171, 93, 234, 194, 124, 220, 57, 110, 14, 116, 48, 211, 23, 69, 72, 115, 71, 80, 216, 114, 76, 132, 2, 110, 12, 239, 55, 160, 215, 180, 148, 68, 167, 51, 136, 110, 202, 105, 211, 203, 104, 168, 105, 160, 123, 220, 171, 197, 117, 82, 196, 141, 228, 46, 1, 110, 90, 135, 148, 74, 177, 216, 232, 58, 249, 207, 246, 249, 88, 5, 113, 171, 89, 26, 242, 139, 26, 5, 47, 97, 36, 194, 207, 153, 132, 44, 2, 111, 41, 135, 133, 115, 5, 70, 255, 71, 215, 223, 206, 141, 121, 201, 103, 84, 223, 230, 191, 118, 201, 173, 125, 46, 183, 254, 69, 147, 55, 114, 184, 36, 252, 0, 131, 223, 2, 214, 74, 236, 239, 229, 32, 80, 33, 247, 114, 246, 228, 8, 100, 107, 245, 72, 191, 225, 105, 203, 191, 93, 9, 139, 177, 41, 147, 143, 158, 68, 224, 236, 86, 243, 76, 94, 203, 181, 57, 139, 87, 224, 180, 211, 248, 155, 149, 163, 197, 221, 231, 168, 162, 24, 61, 89, 94, 57, 62, 221, 71, 216, 162, 222, 168, 97, 236, 100, 25, 231, 236, 145, 150, 141, 189, 96, 62, 255, 211, 47, 240, 76, 124, 142, 33, 104, 116, 0, 126, 0, 47, 102, 1, 10, 200, 92, 130, 84, 22, 74, 44, 60, 61, 88, 67, 19, 230, 24, 156, 35, 159, 54, 185, 29, 50, 162, 90, 6, 166, 65, 234, 95, 48, 96, 251, 161, 29, 214, 164, 119, 141, 243, 57, 33, 150, 254, 240, 51, 232, 73, 13, 134, 132, 39, 3, 99, 44, 21, 243, 32, 92, 58, 11, 144, 68, 178, 98, 100, 21, 99, 89, 239, 239, 211, 203, 149, 254, 204, 198, 23, 53, 148, 52, 254, 57, 175, 158, 200, 228, 58, 49, 113, 224, 75, 22, 26, 108, 27, 3, 114, 163, 152, 247, 244, 59, 130, 1, 115, 122, 20, 178, 194, 166, 242, 126, 216, 60, 157, 44, 45, 182, 233, 35, 87, 109, 223, 113, 248, 62, 223, 184, 168, 142, 171, 82, 94, 40, 207, 182, 82, 30, 234, 251, 57, 212, 84, 185, 164, 22, 160, 62, 177, 203, 24, 232, 232, 176, 128, 174, 98, 202, 133, 161, 73, 207, 161, 116, 237, 245, 15, 19, 9, 115, 185, 159, 176, 28, 59, 109, 230, 90, 254, 189, 78, 73, 88, 88, 153, 116, 104, 66, 125, 175, 38, 92, 52, 127, 25, 134, 38, 133, 90, 32, 237, 41, 121, 139, 191, 59, 112, 66, 63, 192, 209, 173, 97, 113, 27, 243, 0, 74, 10, 8, 208, 156, 240, 18, 10, 203, 215, 164, 62, 118, 12, 45, 178, 134, 166, 193, 132, 37, 140, 1, 178, 124, 198, 136, 144, 240, 200, 108, 64, 12, 186, 157, 122, 27, 165, 93, 234, 34, 49, 80, 126, 212, 208, 35, 140, 60, 243, 220, 102, 118, 250, 115, 73, 76, 198, 28, 86, 139, 23, 104, 83, 51, 39, 125, 32, 27, 175, 174, 125, 91, 232, 1, 144, 39, 160, 81, 219, 64, 111, 85, 172, 24, 165, 138, 138, 123, 98, 185, 34, 100, 226, 40, 118, 13, 79, 206, 156, 122, 38, 119, 181, 184, 7, 179, 182, 253, 43, 88, 47, 251, 67, 203, 187, 245, 70, 62, 238, 39, 105, 244, 234, 59, 37, 151, 243, 116, 222, 193, 245, 16, 21, 89, 157, 7, 195, 150, 229, 69, 216, 155, 179, 227, 201, 2, 111, 181, 148, 161, 224, 202, 242, 104, 120, 28, 204, 247, 109, 144, 156, 232, 25, 171, 131, 146, 19, 239, 231, 207, 138, 166, 143, 36, 45, 151, 44, 86, 138, 131, 53, 55, 186, 180, 37, 71, 14, 21, 236, 188, 117, 210, 212, 250, 41, 176, 85, 8, 180, 177, 15, 210, 223, 192, 171, 18, 72, 79, 140, 124, 201, 28, 3, 41, 125, 100, 230, 226, 10, 232, 51, 150, 151, 188, 216, 166, 153, 28, 116, 78, 202, 14, 200, 194, 37, 205, 219, 245, 90, 4, 122, 116, 253, 76, 86, 244, 218, 201, 134, 137, 212, 103, 105, 255, 33, 166, 30, 113, 183, 82, 200, 148, 155, 184, 243, 189, 6, 123, 76, 232, 192, 100, 213, 235, 8, 137, 247, 77, 184, 157, 189, 148, 234, 6, 203, 104, 97, 149, 60, 221, 139, 77, 210, 91, 128, 71, 86, 121, 225, 134, 180, 107, 193, 148, 160, 149, 3, 107, 180, 195, 89, 63, 7, 67, 40, 39, 164, 6, 110, 157, 46, 210, 157, 70, 153, 225, 169, 26, 240, 87, 248, 207, 100, 174, 22, 239, 36, 211, 16, 180, 56, 29, 129, 34, 28, 58, 193, 252, 236, 10, 115, 18, 28, 70, 32, 105, 82, 74, 53, 3, 253, 2, 233, 45, 70, 67, 166, 33, 213, 58, 41, 90, 198, 154, 196, 127, 40, 178, 232, 227, 225, 106, 60, 87, 149, 172, 140, 148, 32, 66, 194, 168, 106, 193, 86, 105, 195, 208, 104, 120, 249, 63, 157, 207, 60, 55, 162, 14, 69, 123, 214, 87, 205, 114, 208, 103, 138, 43, 46, 85, 90, 57, 24, 131, 228, 214, 165, 243, 164, 144, 182, 54, 160, 74, 103, 86, 112, 154, 88, 87, 116, 161, 66, 5, 164, 140, 147, 160, 11, 106, 221, 211, 82, 177, 6, 113, 86, 239, 222, 217, 247, 161, 164, 28, 99, 158, 98, 49, 172, 214, 100, 181, 61, 101, 2, 92, 167, 171, 183, 99, 239, 140, 162, 5, 38, 210, 148, 247, 130, 138, 195, 59, 159, 27, 216, 244, 220, 152, 229, 147, 223, 238, 196, 28, 126, 13, 5, 5, 255, 92, 172, 31, 128, 129, 211, 188, 152, 218, 197, 60, 190, 188, 215, 26, 123, 28, 41, 104, 29, 198, 143, 120, 163, 39, 251, 131, 58, 93, 248, 230, 84, 74, 115, 157, 33, 228, 95, 105, 184, 234, 246, 244, 82, 75, 83, 196, 254, 130, 8, 42, 175, 73, 175, 121, 229, 53, 101, 45, 38, 10, 200, 87, 208, 60, 223, 92, 205, 125, 129, 6, 78, 199, 196, 199, 227, 24, 238, 244, 85, 117, 32, 193, 30, 101, 221, 14, 83, 155, 146, 242, 57, 167, 247, 13, 148, 152, 104, 122, 211, 230, 26, 161, 195, 140, 47, 147, 172, 145, 130, 62, 249, 181, 170, 217, 88, 192, 75, 162, 118, 156, 184, 85, 63, 147, 192, 65, 149, 85, 118, 0, 113, 232, 248, 183, 49, 18, 120, 128, 173, 200, 45, 188, 225, 87, 44, 196, 132, 229, 95, 130, 193, 171, 113, 106, 177, 82, 106, 77, 194, 190, 96, 31, 97, 240, 195, 253, 196, 50, 91, 88, 160, 40, 36, 73, 225, 186, 56, 122, 1, 39, 200, 8, 215, 234, 226, 47, 243, 182, 157, 8, 49, 237, 221, 43, 151, 195, 47, 152, 167, 80, 250, 231, 255, 143, 58, 65, 10, 237, 3, 200, 224, 60, 12, 15, 26, 48, 134, 74, 140, 187, 237, 160, 179, 226, 14, 158, 104, 213, 59, 222, 21, 98, 6, 161, 4, 87, 238, 115, 194, 88, 53, 239, 179, 188, 202, 245, 148, 84, 149, 136, 242, 1, 153, 249, 73, 177, 7, 84, 16, 217, 242, 2, 135, 229, 54, 11, 78, 28, 117, 154, 232, 135, 84, 67, 246, 180, 156, 71, 116, 127, 250, 42, 177, 46, 220, 40, 41, 166, 213, 182, 25, 134, 80, 123, 114, 237, 28, 121, 125, 67, 100, 59, 203, 127, 102, 18, 237, 115, 120, 37, 242, 55, 218, 136, 7, 95, 139, 79, 184, 87, 12, 179, 237, 48, 227, 245, 166, 121, 71, 214, 250, 57, 28, 139, 27, 81, 135, 203, 88, 78, 229, 14, 188, 3, 125, 131, 173, 75, 182, 9, 114, 136, 78, 146, 172, 201, 103, 11, 97, 238, 181, 85, 31, 232, 165, 209, 0, 112, 121, 205, 26, 93, 144, 217, 22, 143, 86, 239, 223, 32, 181, 239, 163, 1, 186, 244, 163, 32, 228, 0, 35, 146, 140, 19, 122, 247, 118, 7, 126, 125, 65, 163, 14, 188, 90, 182, 212, 222, 104, 42, 46, 47, 107, 209, 171, 164, 199, 186, 39, 27, 237, 69, 203, 241, 130, 176, 148, 170, 79, 141, 178, 70, 141, 194, 51, 148, 159, 207, 201, 67, 245, 64, 203, 181, 73, 187, 7, 149, 177, 188, 126, 242, 121, 98, 88, 20, 185, 65, 0, 54, 150, 46, 5, 174, 33, 120, 115, 95, 24, 159, 128, 144, 214, 92, 38, 169, 122, 204, 243, 34, 141, 81, 31, 204, 30, 128, 190, 233, 177, 4, 79, 81, 230, 104, 235, 112, 13, 37, 107, 1, 10, 110, 56, 250, 157, 13, 171, 48, 235, 129, 253, 229, 220, 71, 87, 187, 162, 177, 25, 42, 140, 65, 103, 15, 44, 237, 199, 187, 188, 204, 53, 186, 67, 236, 4, 194, 7, 75, 80, 242, 10, 61, 126, 71, 245, 12, 231, 177, 56, 182, 159, 95, 189, 240, 128, 225, 65, 133, 99, 207, 148, 142, 232, 101, 224, 235, 190, 130, 136, 252, 251, 154, 22, 31, 170, 224, 234, 137, 161, 95, 209, 241, 94, 33, 122, 83, 201, 125, 116, 69, 121, 61, 212, 229, 95, 96, 85, 209, 115, 242, 234, 245, 138, 233, 224, 207, 26, 97, 216, 9, 228, 167, 216, 69, 36, 34, 102, 186, 237, 78, 165, 189, 65, 11, 223, 47, 237, 213, 168, 117, 23, 119, 52, 40, 206, 58, 214, 75, 5, 6, 156, 210, 229, 151, 253, 156, 56, 145, 138, 221, 58, 177, 16, 147, 96, 112, 69, 5, 27, 197, 51, 19, 148, 64, 120, 107, 231, 7, 88, 154, 82, 191, 4, 239, 165, 168, 5, 139, 144, 19, 31, 173, 169, 195, 27, 37, 51, 106, 114, 78, 154, 143, 138, 120, 9, 79, 72, 198, 81, 8, 75, 125, 65, 117, 159, 233, 46, 210, 187, 88, 82, 103, 70, 53, 80, 134, 4, 244, 39, 60, 133, 86, 124, 174, 54, 145, 71, 32, 29, 136, 177, 30, 124, 216, 182, 229, 32, 41, 136, 28, 39, 196, 97, 62, 242, 60, 98, 117, 115, 44, 166, 25, 70, 196, 66, 149, 174, 210, 143, 253, 219, 10, 58, 245, 169, 253, 153, 185, 135, 253, 220, 186, 96, 151, 81, 38, 10, 12, 110, 116, 187, 62, 175, 151, 220, 18, 13, 94, 192, 234, 13, 58, 20, 3, 195, 59, 198, 31, 65, 50, 201, 225, 194, 45, 86, 5, 110, 1, 184, 87, 98, 17, 17, 130, 12, 145, 153, 64, 187, 10, 160, 17, 180, 2, 213, 254, 229, 210, 35, 74, 134, 145, 192, 1, 216, 217, 185, 203, 250, 45, 235, 162, 232, 190, 146, 159, 97, 121, 225, 135, 249, 85, 117, 68, 121, 124, 41, 111, 149, 217, 193, 189, 217, 0, 11, 247, 118, 30, 19, 253, 119, 208, 144, 173, 203, 32, 72, 54, 84, 73, 142, 156, 66, 103, 32, 222, 28, 139, 119, 200, 80, 60, 6, 127, 141, 93, 60, 32, 24, 212, 198, 81, 177, 49, 101, 190, 171, 5, 80, 170, 15, 75, 195, 160, 66, 179, 134, 126, 254, 141, 222, 237, 225, 82, 148, 146, 0, 106, 74, 7, 152, 53, 56, 221, 124, 25, 226, 122, 219, 255, 232, 56, 85, 190, 172, 179, 99, 244, 228, 192, 43, 34, 101, 38, 251, 200, 58, 129, 183, 106, 58, 218, 60, 206, 168, 152, 182, 167, 245, 53, 21, 82, 157, 54, 154, 154, 62, 70, 95, 219, 64, 253, 71, 171, 157, 59, 104, 40, 229, 121, 238, 42, 33, 64, 80, 112, 32, 53, 57, 34, 52, 31, 140, 170, 227, 143, 173, 29, 3, 19, 220, 210, 90, 161, 38, 236, 217, 89, 56, 95, 105, 61, 57, 18, 27, 29, 32, 187, 244, 129, 243, 92, 234, 149, 121, 232, 165, 75, 88, 153, 194, 220, 214, 190, 78, 191, 109, 89, 84, 41, 33, 137, 0, 141, 61, 63, 85, 153, 198, 216, 77, 82, 208, 76, 108, 12, 142, 217, 157, 234, 75, 229, 136, 47, 222, 20, 254, 156, 239, 107, 222, 173, 226, 153, 70, 117, 249, 164, 205, 163, 177, 203, 194, 182, 162, 128, 208, 235, 243, 143, 194, 117, 67, 46, 227, 230, 229, 211, 5, 126, 67, 212, 95, 160, 216, 237, 204, 37, 122, 72, 79, 191, 28, 59, 5, 78, 169, 248, 241, 163, 121, 31, 152, 249, 248, 99, 245, 43, 183, 154, 117, 167, 118, 7, 159, 101, 124, 211, 14, 95, 78, 169, 150, 213, 127, 114, 18, 141, 0, 171, 180, 227, 9, 212, 9, 148, 62, 57, 93, 28, 197, 196, 102, 17, 190, 168, 189, 186, 243, 121, 71, 101, 236, 204, 36, 14, 110, 54, 175, 57, 152, 115, 219, 44, 0, 75, 232, 128, 190, 182, 170, 163, 233, 175, 42, 7, 235, 14, 42, 41, 68, 156, 85, 100, 172, 209, 200, 34, 8, 217, 156, 1, 205, 193, 83, 133, 146, 224, 193, 196, 50, 74, 62, 168, 251, 164, 157, 150, 146, 153, 179, 39, 255, 109, 221, 50, 34, 89, 12, 161, 108, 57, 241, 183, 17, 123, 23, 59, 110, 52, 252, 200, 115, 226, 63, 234, 10, 212, 250, 12, 14, 76, 188, 211, 205, 31, 248, 165, 147, 104, 173, 10, 180, 232, 196, 19, 1, 102, 111, 12, 123, 61, 168, 101, 243, 94, 51, 170, 98, 130, 138, 8, 225, 211, 40, 205, 155, 142, 246, 89, 152, 251, 241, 92, 163, 116, 147, 176, 14, 198, 59, 26, 143, 80, 145, 51, 94, 36, 57, 97, 139, 159, 31, 173, 165, 163, 223, 83, 84, 176, 7, 73, 60, 7, 99, 195, 32, 151, 65, 246, 52, 54, 76, 4, 18, 60, 113, 233, 24, 197, 75, 57, 120, 150, 231, 243, 205, 104, 60, 160, 91, 113, 245, 211, 150, 33, 29, 125, 92, 51, 234, 202, 71, 154, 14, 21, 220, 166, 139, 124, 35, 43, 189, 38, 193, 31, 104, 250, 207, 215, 244, 168, 149, 225, 159, 248, 58, 255, 151, 62, 221, 62, 223, 210, 27, 141, 165, 215, 71, 50, 46, 179, 243, 244, 133, 60, 27, 155, 161, 41, 178, 140, 189, 84, 110, 227, 75, 192, 102, 82, 134, 87, 103, 77, 76, 200, 130, 246, 137, 247, 196, 198, 145, 59, 67, 53, 163, 216, 157, 83, 218, 210, 95, 144, 235, 106, 124, 14, 108, 217, 21, 232, 34, 43, 166, 122, 79, 103, 205, 129, 109, 33, 226, 148, 13, 240, 189, 113, 146, 130, 215, 17, 186, 79, 199, 151, 19, 143, 98, 211, 225, 172, 4, 61, 250, 192, 76, 52, 205, 205, 92, 38, 165, 41, 173, 78, 227, 27, 65, 142, 78, 10, 2, 69, 156, 64, 79, 170, 198, 248, 113, 37, 195, 118, 189, 11, 48, 215, 128, 222, 180, 245, 152, 128, 42, 44, 150, 97, 26, 78, 224, 241, 5, 107, 190, 98, 155, 205, 24, 211, 32, 108, 219, 164, 241, 29, 120, 33, 65, 243, 232, 222, 58, 130, 6, 131, 142, 196, 253, 5, 131, 231, 35, 116, 32, 113, 196, 181, 226, 111, 18, 175, 106, 47, 56, 27, 144, 244, 87, 146, 32, 106, 248, 240, 132, 0, 131, 20, 140, 23, 96, 241, 1, 37, 94, 222, 72, 15, 121, 228, 214, 118, 253, 185, 2, 191, 126, 39, 138, 36, 37, 81, 201, 48, 53, 161, 53, 115, 46, 84, 182, 35, 2, 88, 207, 17, 86, 96, 154, 107, 240, 16, 190, 108, 189, 197, 2, 37, 22, 191, 170, 235, 188, 83, 66, 189, 33, 165, 205, 185, 144, 81, 109, 136, 114, 131, 126, 20, 52, 178, 90, 202, 184, 184, 76, 158, 47, 16, 212, 210, 93, 111, 123, 177, 210, 89, 96, 100, 13, 181, 145, 73, 49, 253, 155, 55, 200, 15, 207, 70, 24, 89, 196, 109, 250, 209, 232, 185, 187, 247, 186, 63, 163, 70, 225, 10, 103, 8, 233, 249, 53, 42, 244, 161, 202, 126, 87, 40, 86, 247, 17, 56, 172, 34, 13, 166, 83, 145, 151, 246, 244, 231, 9, 2, 54, 68, 64, 216, 144, 181, 113, 174, 135, 195, 8, 136, 7, 208, 44, 243, 122, 50, 88, 93, 109, 190, 193, 124, 144, 233, 247, 179, 38, 45, 118, 244, 191, 254, 0, 52, 176, 167, 174, 50, 214, 240, 133, 66, 47, 20, 14, 200, 19, 122, 161, 113, 105, 237, 72, 255, 23, 160, 11, 184, 52, 231, 125, 169, 92, 136, 213, 81, 149, 2, 30, 53, 71, 24, 253, 249, 75, 238, 204, 159, 197, 122, 170, 74, 125, 61, 166, 216, 191, 73, 161, 70, 79, 177, 8, 48, 62, 127, 148, 196, 151, 153, 56, 213, 48, 112, 242, 20, 91, 60, 104, 140, 71, 45, 239, 9, 94, 111, 86, 146, 200, 186, 237, 168, 116, 1, 61, 107, 95, 153, 94, 50, 233, 31, 19, 113, 56, 61, 99, 113, 110, 14, 150, 230, 152, 167, 185, 238, 121, 38, 208, 146, 174, 228, 30, 152, 145, 31, 27, 158, 125, 209, 223, 24, 48, 146, 87, 184, 222, 178, 246, 130, 121, 104, 108, 1, 119, 59, 211, 50, 197, 59, 229, 19, 225, 228, 206, 167, 198, 246, 143, 60, 195, 164, 48, 239, 0, 155, 153, 241, 113, 188, 137, 217, 165, 139, 43, 102, 14, 121, 45, 146, 198, 19, 22, 207, 234, 168, 235, 219, 205, 198, 249, 31, 209, 35, 24, 80, 135, 65, 82, 102, 183, 86, 134, 168, 4, 177, 247, 157, 234, 180, 209, 240, 93, 168, 136, 122, 254, 158, 72, 29, 187, 190, 17, 215, 161, 11, 198, 37, 213, 14, 162, 112, 10, 176, 79, 47, 157, 108, 28, 17, 129, 201, 20, 222, 67, 72, 101, 79, 190, 4, 199, 89, 131, 111, 65, 189, 141, 128, 226, 81, 88, 118, 167, 7, 76, 158, 52, 19, 254, 141, 240, 42, 85, 115, 170, 114, 209, 161, 73, 255, 114, 170, 105, 49, 218, 72, 103, 111, 105, 86, 161, 73, 187, 84, 165, 34, 102, 174, 106, 99, 190, 247, 187, 55, 16, 31, 68, 189, 144, 198, 57, 32, 16, 4, 57, 210, 30, 238, 213, 64, 54, 223, 90, 82, 73, 109, 229, 238, 53, 131, 94, 126, 32, 12, 176, 55, 247, 190, 220, 203, 120, 114, 211, 87, 198, 234, 16, 12, 147, 57, 72, 25, 40, 102, 147, 146, 86, 197, 136, 151, 193, 110, 155, 24, 34, 46, 219, 109, 161, 139, 59, 4, 232, 53, 17, 68, 12, 47, 108, 207, 120, 219, 136, 125, 12, 78, 82, 98, 79, 77, 149, 237, 188, 144, 25, 134, 151, 56, 163, 218, 150, 152, 144, 33, 209, 62, 191, 89, 64, 90, 78, 14, 28, 204, 16, 129, 52, 89, 200, 216, 89, 254, 213, 44, 209, 111, 246, 131, 36, 39, 194, 246, 134, 163, 200, 83, 169, 164, 140, 82, 52, 103, 226, 103, 28, 235, 252, 199, 103, 168, 171, 12, 55, 33, 192, 173, 180, 226, 189, 113, 79, 97, 78, 33, 157, 237, 41, 13, 2, 216, 228, 110, 187, 238, 245, 222, 101, 147, 218, 36, 196, 191, 58, 186, 14, 23, 11, 150, 164, 208, 104, 166, 192, 226, 163, 179, 154, 131, 234, 0, 145, 192, 76, 90, 8, 26, 144, 188, 226, 224, 71, 83, 109, 48, 175, 81, 38, 160, 98, 59, 66, 24, 89, 150, 135, 138, 249, 105, 137, 235, 94, 47, 209, 102, 73, 237, 162, 222, 68, 56, 139, 180, 107, 193, 93, 223, 169, 21, 251, 174, 150, 140, 66, 153, 180, 91, 124, 86, 221, 62, 212, 34, 167, 226, 253, 205, 59, 25, 85, 80, 231, 225, 160, 65, 107, 45, 163, 227, 50, 230, 245, 29, 190, 147, 126, 186, 54, 76, 81, 42, 73, 2, 68, 222, 48, 87, 35, 158, 49, 123, 126, 202, 189, 88, 87, 117, 169, 207, 160, 152, 67, 53, 5, 67, 30, 244, 187, 102, 105, 0, 196, 15, 145, 103, 127, 128, 220, 250, 112, 205, 100, 111, 181, 181, 250, 147, 251, 24, 235, 217, 68, 233, 251, 179, 229, 193, 165, 113, 105, 185, 152, 218, 158, 125, 10, 18, 3, 73, 41, 104, 10, 48, 100, 129, 89, 74, 97, 1, 49, 127, 242, 2, 14, 78, 175, 85, 215, 51, 81, 238, 100, 104, 158, 152, 71, 47, 12, 13, 229, 183, 149, 73, 122, 175, 33, 225, 20, 253, 91, 184, 77, 35, 25, 145, 248, 92, 92, 76, 119, 183, 163, 33, 224, 9, 11, 3, 100, 160, 9, 152, 34, 21, 85, 120, 237, 107, 120, 225, 131, 217, 112, 93, 192, 139, 45, 58, 104, 91, 251, 23, 106, 113, 115, 47, 95, 185, 125, 92, 41, 2, 97, 119, 178, 107, 124, 188, 231, 206, 19, 120, 66, 62, 143, 135, 56, 168, 58, 60, 16, 139, 17, 50, 63, 145, 103, 20, 193, 92, 227, 128, 6, 171, 144, 98, 126, 173, 147, 45, 7, 218, 217, 13, 96, 103, 231, 175, 96, 215, 173, 69, 245, 148, 45, 135, 44, 131, 240, 16, 86, 11, 25, 220, 183, 208, 121, 224, 209, 243, 162, 100, 46, 38, 218, 43, 68, 167, 66, 67, 19, 184, 140, 14, 29, 244, 153, 222, 255, 26, 3, 112, 178, 46, 184, 177, 12, 23, 42, 144, 174, 239, 149, 93, 215, 228, 70, 69, 170, 239, 130, 10, 78, 192, 95, 52, 83, 134, 10, 219, 45, 240, 14, 1, 216, 138, 91, 165, 193, 143, 44, 231, 43, 173, 98, 116, 238, 131, 46, 69, 167, 103, 52, 198, 54, 159, 105, 51, 131, 4, 151, 219, 78, 1, 227, 201, 69, 210, 26, 41, 30, 197, 25, 102, 6, 20, 74, 169, 187, 53, 118, 175, 122, 158, 124, 131, 79, 62, 15];

    let etalon_result = [8, 0, 0, 2, 0, 0, 11, 0, 16, 109, 0, 0, 16, 105, 0, 5, 229, 48, 130, 5, 225, 48, 130, 4, 201, 160, 3, 2, 1, 2, 2, 16,
        67, 88, 213, 1, 149, 130, 101, 119, 18, 166, 220, 152, 142, 57, 73, 10, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 48,
        59, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 30, 48, 28, 6, 3, 85, 4, 10, 19, 21, 71, 111, 111, 103, 108, 101, 32,
        84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 49, 12, 48, 10, 6, 3, 85, 4, 3, 19, 3, 87, 82, 50, 48, 30, 23, 13, 50, 53, 48, 53, 49, 50, 48, 56, 52, 52, 48, 49, 90, 23, 13, 50, 53, 48, 56, 48, 52, 48, 56, 52, 52, 48, 48, 90, 48, 34, 49, 32, 48, 30, 6, 3, 85, 4, 3, 19, 23, 117, 112, 108, 111, 97, 100, 46, 118, 105, 100, 101, 111, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 48, 89, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61, 3, 1, 7, 3, 66, 0, 4, 109, 122, 96, 117, 1, 4, 176, 94, 63, 84, 238, 1, 98, 27, 211, 232, 111, 92, 206, 94, 6, 9, 211, 183, 77, 11, 212, 131, 73, 171, 73, 29, 23, 157, 226, 51, 0, 187, 218, 54, 131, 176, 184, 33, 119, 152, 153, 198, 3, 95, 198, 211, 57, 176, 242, 48, 45, 207, 209, 243, 141, 224, 156, 227, 163, 130, 3, 195, 48, 130, 3, 191, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 7, 128, 48, 19, 6, 3, 85, 29, 37, 4, 12, 48, 10, 6, 8, 43, 6, 1, 5, 5, 7, 3, 1, 48, 12, 6, 3, 85, 29, 19, 1, 1, 255, 4, 2, 48, 0, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 141, 73, 128, 34, 106, 71, 87, 172, 175, 59, 248, 196, 248, 93, 114, 119, 94, 112, 162, 85, 48, 31, 6, 3, 85, 29, 35, 4, 24, 48, 22, 128, 20, 222, 27, 30, 237, 121, 21, 212, 62, 55, 36, 195, 33, 187, 236, 52, 57, 109, 66, 178, 48, 48, 88, 6, 8, 43, 6, 1, 5, 5, 7, 1, 1, 4, 76, 48, 74, 48, 33, 6, 8, 43, 6, 1, 5, 5, 7, 48, 1, 134, 21, 104, 116, 116, 112, 58, 47, 47, 111, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 119, 114, 50, 48, 37, 6, 8, 43, 6, 1, 5, 5, 7, 48, 2, 134, 25, 104, 116, 116, 112, 58, 47, 47, 105, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 119, 114, 50, 46, 99, 114, 116, 48, 130, 1, 152, 6, 3, 85, 29, 17, 4, 130, 1, 143, 48, 130, 1, 139, 130, 23, 117, 112, 108, 111, 97, 100, 46, 118, 105, 100, 101, 111, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 20, 42, 46, 99, 108, 105, 101, 110, 116, 115, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 17, 42, 46, 100, 111, 99, 115, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 18, 42, 46, 100, 114, 105, 118, 101, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 19, 42, 46, 103, 100, 97, 116, 97, 46, 121, 111, 117, 116, 117, 98, 101, 46, 99, 111, 109, 130, 16, 42, 46, 103, 111, 111, 103, 108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 130, 19, 42, 46, 112, 104, 111, 116, 111, 115, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 23, 42, 46, 121, 111, 117, 116, 117, 98, 101, 45, 51, 114, 100, 45, 112, 97, 114, 116, 121, 46, 99, 111, 109, 130, 17, 117, 112, 108, 111, 97, 100, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 19, 42, 46, 117, 112, 108, 111, 97, 100, 46, 103, 111, 111, 103, 108, 101, 46, 99, 111, 109, 130, 18, 117, 112, 108, 111, 97, 100, 46, 121, 111, 117, 116, 117, 98, 101, 46, 99, 111, 109, 130, 20, 42, 46, 117, 112, 108, 111, 97, 100, 46, 121, 111, 117, 116, 117, 98, 101, 46, 99, 111, 109, 130, 31, 117, 112, 108, 111, 97, 100, 115, 46, 115, 116, 97, 103, 101, 46, 103, 100, 97, 116, 97, 46, 121, 111, 117, 116, 117, 98, 101, 46, 99, 111, 109, 130, 21, 98, 103, 45, 99, 97, 108, 108, 45, 100, 111, 110, 97, 116, 105, 111, 110, 46, 103, 111, 111, 103, 130, 27, 98, 103, 45, 99, 97, 108, 108, 45, 100, 111, 110, 97, 116, 105, 111, 110, 45, 97, 108, 112, 104, 97, 46, 103, 111, 111, 103, 130, 28, 98, 103, 45, 99, 97, 108, 108, 45, 100, 111, 110, 97, 116, 105, 111, 110, 45, 99, 97, 110, 97, 114, 121, 46, 103, 111, 111, 103, 130, 25, 98, 103, 45, 99, 97, 108, 108, 45, 100, 111, 110, 97, 116, 105, 111, 110, 45, 100, 101, 118, 46, 103, 111, 111, 103, 48, 19, 6, 3, 85, 29, 32, 4, 12, 48, 10, 48, 8, 6, 6, 103, 129, 12, 1, 2, 1, 48, 54, 6, 3, 85, 29, 31, 4, 47, 48, 45, 48, 43, 160, 41, 160, 39, 134, 37, 104, 116, 116, 112, 58, 47, 47, 99, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 119, 114, 50, 47, 111, 81, 54, 110, 121, 114, 56, 70, 48, 109, 48, 46, 99, 114, 108, 48, 130, 1, 5, 6, 10, 43, 6, 1, 4, 1, 214, 121, 2, 4, 2, 4, 129, 246, 4, 129, 243, 0, 241, 0, 119, 0, 18, 241, 78, 52, 189, 83, 114, 76, 132, 6, 25, 195, 143, 63, 122, 19, 248, 231, 181, 98, 135, 136, 156, 109, 48, 5, 132, 235, 229, 134, 38, 58, 0, 0, 1, 150, 195, 225, 70, 160, 0, 0, 4, 3, 0, 72, 48, 70, 2, 33, 0, 150, 124, 237, 110, 175, 251, 9, 27, 137, 159, 61, 108, 142, 223, 241, 83, 50, 108, 127, 75, 184, 199, 177, 148, 207, 114, 128, 27, 213, 254, 33, 174, 2, 33, 0, 132, 204, 208, 195, 188, 252, 83, 6, 41, 165, 2, 120, 216, 151, 131, 67, 97, 148, 202, 238, 23, 191, 208, 240, 143, 99, 244, 155, 101, 198, 238, 72, 0, 118, 0, 125, 89, 30, 18, 225, 120, 42, 123, 28, 97, 103, 124, 94, 253, 248, 208, 135, 92, 20, 160, 78, 149, 158, 185, 3, 47, 217, 14, 140, 46, 121, 184, 0, 0, 1, 150, 195, 225, 73, 9, 0, 0, 4, 3, 0, 71, 48, 69, 2, 33, 0, 166, 25, 83, 25, 53, 28, 163, 191, 5, 168, 123, 140, 131, 81, 236, 75, 153, 134, 79, 250, 187, 220, 36, 197, 120, 155, 125, 112, 153, 229, 150, 60, 2, 32, 0, 155, 66, 233, 126, 115, 186, 51, 205, 56, 4, 177, 160, 50, 228, 127, 167, 220, 151, 221, 217, 190, 107, 9, 171, 128, 47, 195, 84, 24, 70, 120, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 3, 130, 1, 1, 0, 146, 226, 213, 159, 37, 70, 145, 189, 106, 123, 15, 73, 195, 214, 19, 171, 77, 212, 99, 250, 209, 240, 59, 105, 111, 51, 55, 217, 82, 248, 245, 174, 168, 185, 177, 225, 19, 23, 162, 95, 35, 75, 117, 146, 14, 149, 142, 123, 11, 213, 17, 50, 212, 68, 159, 39, 42, 114, 154, 165, 36, 245, 202, 114, 124, 95, 124, 117, 239, 121, 62, 162, 22, 27, 104, 102, 245, 6, 200, 211, 73, 230, 37, 158, 172, 241, 229, 213, 26, 133, 81, 223, 11, 23, 151, 21, 30, 70, 73, 156, 72, 152, 65, 131, 137, 109, 43, 224, 91, 62, 107, 83, 117, 163, 105, 161, 151, 138, 147, 153, 101, 237, 182, 34, 180, 122, 231, 82, 26, 138, 182, 19, 121, 103, 154, 61, 63, 19, 135, 41, 28, 217, 144, 92, 83, 156, 22, 111, 114, 105, 153, 229, 41, 202, 215, 219, 1, 115, 33, 141, 167, 38, 2, 80, 19, 202, 60, 115, 52, 3, 138, 247, 211, 53, 144, 86, 117, 9, 162, 102, 250, 139, 20, 208, 173, 91, 84, 61, 41, 55, 63, 135, 247, 27, 175, 16, 50, 99, 56, 221, 173, 81, 59, 76, 195, 142, 190, 220, 234, 118, 247, 249, 141, 226, 111, 42, 202, 151, 144, 25, 3, 2, 214, 55, 42, 137, 66, 138, 60, 178, 32, 94, 160, 141, 25, 1, 169, 247, 149, 18, 221, 92, 194, 170, 159, 184, 65, 10, 12, 59, 206, 4, 173, 73, 157, 250, 0, 0, 0, 5, 15, 48, 130, 5, 11, 48, 130, 2, 243, 160, 3, 2, 1, 2, 2, 16, 127, 240, 5, 160, 124, 76, 222, 209, 0, 173, 157, 102, 165, 16, 123, 152, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 49, 48, 30, 23, 13, 50, 51, 49, 50, 49, 51, 48, 57, 48, 48, 48, 48, 90, 23, 13, 50, 57, 48, 50, 50, 48, 49, 52, 48, 48, 48, 48, 90, 48, 59, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 30, 48, 28, 6, 3, 85, 4, 10, 19, 21, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 49, 12, 48, 10, 6, 3, 85, 4, 3, 19, 3, 87, 82, 50, 48, 130, 1, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 1, 15, 0, 48, 130, 1, 10, 2, 130, 1, 1, 0, 169, 255, 156, 127, 69, 30, 112, 168, 83, 159, 202, 217, 229, 13, 222, 70, 87, 87, 125, 188, 143, 154, 90, 172, 70, 241, 132, 154, 187, 145, 219, 201, 251, 47, 1, 251, 146, 9, 0, 22, 94, 160, 28, 248, 193, 171, 249, 120, 47, 74, 204, 216, 133, 162, 216, 89, 60, 14, 211, 24, 251, 177, 245, 36, 13, 38, 238, 182, 91, 100, 118, 124, 20, 199, 47, 122, 206, 168, 76, 183, 244, 217, 8, 252, 223, 135, 35, 53, 32, 168, 226, 105, 226, 140, 78, 63, 177, 89, 250, 96, 162, 30, 179, 201, 32, 83, 25, 130, 202, 54, 83, 109, 96, 77, 233, 0, 145, 252, 118, 141, 92, 8, 15, 10, 194, 220, 241, 115, 107, 197, 19, 110, 10, 79, 122, 194, 242, 2, 28, 46, 180, 99, 131, 218, 49, 246, 45, 117, 48, 178, 251, 171, 194, 110, 219, 169, 192, 14, 185, 249, 103, 212, 195, 37, 87, 116, 235, 5, 180, 233, 142, 181, 222, 40, 205, 204, 122, 20, 228, 113, 3, 203, 77, 97, 46, 97, 87, 197, 25, 169, 11, 152, 132, 26, 232, 121, 41, 217, 178, 141, 47, 255, 87, 106, 102, 224, 206, 171, 149, 168, 41, 150, 99, 112, 18, 103, 30, 58, 225, 219, 176, 33, 113, 215, 124, 158, 253, 170, 23, 110, 254, 43, 251, 56, 23, 20, 209, 102, 167, 175, 154, 181, 112, 204, 200, 99, 129, 58, 140, 192, 42, 169, 118, 55, 206, 227, 2, 3, 1, 0, 1, 163, 129, 254, 48, 129, 251, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 29, 6, 3, 85, 29, 37, 4, 22, 48, 20, 6, 8, 43, 6, 1, 5, 5, 7, 3, 1, 6, 8, 43, 6, 1, 5, 5, 7, 3, 2, 48, 18, 6, 3, 85, 29, 19, 1, 1, 255, 4, 8, 48, 6, 1, 1, 255, 2, 1, 0, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 222, 27, 30, 237, 121, 21, 212, 62, 55, 36, 195, 33, 187, 236, 52, 57, 109, 66, 178, 48, 48, 31, 6, 3, 85, 29, 35, 4, 24, 48, 22, 128, 20, 228, 175, 43, 38, 113, 26, 43, 72, 39, 133, 47, 82, 102, 44, 239, 240, 137, 19, 113, 62, 48, 52, 6, 8, 43, 6, 1, 5, 5, 7, 1, 1, 4, 40, 48, 38, 48, 36, 6, 8, 43, 6, 1, 5, 5, 7, 48, 2, 134, 24, 104, 116, 116, 112, 58, 47, 47, 105, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 114, 49, 46, 99, 114, 116, 48, 43, 6, 3, 85, 29, 31, 4, 36, 48, 34, 48, 32, 160, 30, 160, 28, 134, 26, 104, 116, 116, 112, 58, 47, 47, 99, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 114, 47, 114, 49, 46, 99, 114, 108, 48, 19, 6, 3, 85, 29, 32, 4, 12, 48, 10, 48, 8, 6, 6, 103, 129, 12, 1, 2, 1, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 3, 130, 2, 1, 0, 69, 117, 139, 229, 31, 59, 68, 19, 150, 26, 171, 88, 241, 53, 201, 111, 61, 210, 208, 51, 74, 134, 51, 186, 87, 81, 79, 238, 196, 52, 218, 22, 18, 76, 191, 19, 159, 13, 212, 84, 233, 72, 121, 192, 48, 60, 148, 37, 242, 26, 244, 186, 50, 148, 182, 51, 114, 11, 133, 238, 9, 17, 37, 52, 148, 225, 111, 66, 219, 130, 155, 123, 127, 42, 154, 169, 255, 127, 169, 210, 222, 74, 32, 203, 179, 251, 3, 3, 184, 248, 7, 5, 218, 89, 146, 47, 24, 70, 152, 206, 175, 114, 190, 36, 38, 177, 30, 0, 77, 189, 8, 173, 147, 65, 68, 10, 187, 199, 213, 1, 133, 191, 147, 87, 227, 223, 116, 18, 83, 14, 17, 37, 211, 155, 220, 222, 203, 39, 110, 179, 194, 185, 51, 98, 57, 194, 224, 53, 225, 91, 167, 9, 46, 25, 203, 145, 42, 118, 92, 241, 223, 202, 35, 132, 64, 165, 111, 255, 154, 65, 224, 181, 239, 50, 209, 133, 174, 175, 37, 9, 240, 98, 197, 110, 194, 200, 110, 50, 253, 184, 218, 226, 206, 74, 145, 74, 243, 133, 85, 78, 177, 117, 214, 72, 51, 47, 111, 132, 217, 18, 92, 159, 212, 113, 152, 99, 37, 141, 105, 92, 10, 107, 125, 242, 65, 189, 232, 187, 143, 228, 34, 215, 157, 101, 69, 232, 76, 10, 135, 218, 233, 96, 102, 136, 14, 31, 199, 225, 78, 86, 197, 118, 255, 180, 122, 87, 105, 242, 2, 34, 9, 38, 65, 29, 218, 116, 162, 229, 41, 243, 196, 154, 229, 93, 214, 170, 122, 253, 225, 183, 43, 102, 56, 251, 232, 41, 102, 186, 239, 160, 19, 47, 248, 115, 126, 240, 218, 64, 17, 28, 93, 221, 143, 166, 252, 190, 219, 190, 86, 248, 50, 156, 31, 65, 65, 109, 126, 182, 197, 235, 198, 139, 54, 183, 23, 140, 157, 207, 25, 122, 52, 159, 33, 147, 196, 126, 116, 53, 210, 170, 253, 76, 109, 20, 245, 201, 176, 121, 91, 73, 60, 243, 191, 23, 72, 232, 239, 154, 38, 19, 12, 135, 242, 115, 214, 156, 197, 82, 107, 99, 247, 50, 144, 120, 169, 107, 235, 94, 214, 147, 161, 191, 188, 24, 61, 139, 89, 246, 138, 198, 5, 94, 82, 24, 226, 102, 224, 218, 193, 220, 173, 90, 37, 170, 244, 69, 252, 241, 11, 120, 164, 175, 176, 242, 115, 164, 48, 168, 52, 193, 83, 127, 66, 150, 229, 72, 65, 235, 144, 70, 12, 6, 220, 203, 146, 198, 94, 243, 68, 68, 67, 70, 41, 70, 160, 166, 252, 185, 142, 57, 39, 57, 177, 90, 226, 177, 173, 252, 19, 255, 142, 252, 38, 225, 212, 254, 132, 241, 80, 90, 142, 151, 107, 45, 42, 121, 251, 64, 100, 234, 243, 61, 189, 91, 225, 160, 4, 176, 151, 72, 28, 66, 245, 234, 90, 28, 205, 38, 200, 81, 255, 20, 153, 103, 137, 114, 95, 29, 236, 173, 90, 221, 0, 0, 0, 5, 102, 48, 130, 5, 98, 48, 130, 4, 74, 160, 3, 2, 1, 2, 2, 16, 119, 189, 13, 108, 219, 54, 249, 26, 234, 33, 15, 196, 240, 88, 211, 13, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 48, 87, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 66, 69, 49, 25, 48, 23, 6, 3, 85, 4, 10, 19, 16, 71, 108, 111, 98, 97, 108, 83, 105, 103, 110, 32, 110, 118, 45, 115, 97, 49, 16, 48, 14, 6, 3, 85, 4, 11, 19, 7, 82, 111, 111, 116, 32, 67, 65, 49, 27, 48, 25, 6, 3, 85, 4, 3, 19, 18, 71, 108, 111, 98, 97, 108, 83, 105, 103, 110, 32, 82, 111, 111, 116, 32, 67, 65, 48, 30, 23, 13, 50, 48, 48, 54, 49, 57, 48, 48, 48, 48, 52, 50, 90, 23, 13, 50, 56, 48, 49, 50, 56, 48, 48, 48, 48, 52, 50, 90, 48, 71, 49, 11, 48, 9, 6, 3, 85, 4, 6, 19, 2, 85, 83, 49, 34, 48, 32, 6, 3, 85, 4, 10, 19, 25, 71, 111, 111, 103, 108, 101, 32, 84, 114, 117, 115, 116, 32, 83, 101, 114, 118, 105, 99, 101, 115, 32, 76, 76, 67, 49, 20, 48, 18, 6, 3, 85, 4, 3, 19, 11, 71, 84, 83, 32, 82, 111, 111, 116, 32, 82, 49, 48, 130, 2, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 2, 15, 0, 48, 130, 2, 10, 2, 130, 2, 1, 0, 182, 17, 2, 139, 30, 227, 161, 119, 155, 59, 220, 191, 148, 62, 183, 149, 167, 64, 60, 161, 253, 130, 249, 125, 50, 6, 130, 113, 246, 246, 140, 127, 251, 232, 219, 188, 106, 46, 151, 151, 163, 140, 75, 249, 43, 246, 177, 249, 206, 132, 29, 177, 249, 197, 151, 222, 239, 185, 242, 163, 233, 188, 18, 137, 94, 167, 170, 82, 171, 248, 35, 39, 203, 164, 177, 156, 99, 219, 215, 153, 126, 240, 10, 94, 235, 104, 166, 244, 198, 90, 71, 13, 77, 16, 51, 227, 78, 177, 19, 163, 200, 24, 108, 75, 236, 252, 9, 144, 223, 157, 100, 41, 37, 35, 7, 161, 180, 210, 61, 46, 96, 224, 207, 210, 9, 135, 187, 205, 72, 240, 77, 194, 194, 122, 136, 138, 187, 186, 207, 89, 25, 214, 175, 143, 176, 7, 176, 158, 49, 241, 130, 193, 192, 223, 46, 166, 109, 108, 25, 14, 181, 216, 126, 38, 26, 69, 3, 61, 176, 121, 164, 148, 40, 173, 15, 127, 38, 229, 168, 8, 254, 150, 232, 60, 104, 148, 83, 238, 131, 58, 136, 43, 21, 150, 9, 178, 224, 122, 140, 46, 117, 214, 156, 235, 167, 86, 100, 143, 150, 79, 104, 174, 61, 151, 194, 132, 143, 192, 188, 64, 192, 11, 92, 189, 246, 135, 179, 53, 108, 172, 24, 80, 127, 132, 224, 76, 205, 146, 211, 32, 233, 51, 188, 82, 153, 175, 50, 181, 41, 179, 37, 42, 180, 72, 249, 114, 225, 202, 100, 247, 230, 130, 16, 141, 232, 157, 194, 138, 136, 250, 56, 102, 138, 252, 99, 249, 1, 249, 120, 253, 123, 92, 119, 250, 118, 135, 250, 236, 223, 177, 14, 121, 149, 87, 180, 189, 38, 239, 214, 1, 209, 235, 22, 10, 187, 142, 11, 181, 197, 197, 138, 85, 171, 211, 172, 234, 145, 75, 41, 204, 25, 164, 50, 37, 78, 42, 241, 101, 68, 208, 2, 206, 170, 206, 73, 180, 234, 159, 124, 131, 176, 64, 123, 231, 67, 171, 167, 108, 163, 143, 125, 137, 129, 250, 76, 165, 255, 213, 142, 195, 206, 75, 224, 181, 216, 179, 142, 69, 207, 118, 192, 237, 64, 43, 253, 83, 15, 176, 167, 213, 59, 13, 177, 138, 162, 3, 222, 49, 173, 204, 119, 234, 111, 123, 62, 214, 223, 145, 34, 18, 230, 190, 250, 216, 50, 252, 16, 99, 20, 81, 114, 222, 93, 214, 22, 147, 189, 41, 104, 51, 239, 58, 102, 236, 7, 138, 38, 223, 19, 215, 87, 101, 120, 39, 222, 94, 73, 20, 0, 162, 0, 127, 154, 168, 33, 182, 169, 177, 149, 176, 165, 185, 13, 22, 17, 218, 199, 108, 72, 60, 64, 224, 126, 13, 90, 205, 86, 60, 209, 151, 5, 185, 203, 75, 237, 57, 75, 156, 196, 63, 210, 85, 19, 110, 36, 176, 214, 113, 250, 244, 193, 186, 204, 237, 27, 245, 254, 129, 65, 216, 0, 152, 61, 58, 200, 174, 122, 152, 55, 24, 5, 149, 2, 3, 1, 0, 1, 163, 130, 1, 56, 48, 130, 1, 52, 48, 14, 6, 3, 85, 29, 15, 1, 1, 255, 4, 4, 3, 2, 1, 134, 48, 15, 6, 3, 85, 29, 19, 1, 1, 255, 4, 5, 48, 3, 1, 1, 255, 48, 29, 6, 3, 85, 29, 14, 4, 22, 4, 20, 228, 175, 43, 38, 113, 26, 43, 72, 39, 133, 47, 82, 102, 44, 239, 240, 137, 19, 113, 62, 48, 31, 6, 3, 85, 29, 35, 4, 24, 48, 22, 128, 20, 96, 123, 102, 26, 69, 13, 151, 202, 137, 80, 47, 125, 4, 205, 52, 168, 255, 252, 253, 75, 48, 96, 6, 8, 43, 6, 1, 5, 5, 7, 1, 1, 4, 84, 48, 82, 48, 37, 6, 8, 43, 6, 1, 5, 5, 7, 48, 1, 134, 25, 104, 116, 116, 112, 58, 47, 47, 111, 99, 115, 112, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 103, 115, 114, 49, 48, 41, 6, 8, 43, 6, 1, 5, 5, 7, 48, 2, 134, 29, 104, 116, 116, 112, 58, 47, 47, 112, 107, 105, 46, 103, 111, 111, 103, 47, 103, 115, 114, 49, 47, 103, 115, 114, 49, 46, 99, 114, 116, 48, 50, 6, 3, 85, 29, 31, 4, 43, 48, 41, 48, 39, 160, 37, 160, 35, 134, 33, 104, 116, 116, 112, 58, 47, 47, 99, 114, 108, 46, 112, 107, 105, 46, 103, 111, 111, 103, 47, 103, 115, 114, 49, 47, 103, 115, 114, 49, 46, 99, 114, 108, 48, 59, 6, 3, 85, 29, 32, 4, 52, 48, 50, 48, 8, 6, 6, 103, 129, 12, 1, 2, 1, 48, 8, 6, 6, 103, 129, 12, 1, 2, 2, 48, 13, 6, 11, 43, 6, 1, 4, 1, 214, 121, 2, 5, 3, 2, 48, 13, 6, 11, 43, 6, 1, 4, 1, 214, 121, 2, 5, 3, 3, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 11, 5, 0, 3, 130, 1, 1, 0, 52, 164, 30, 177, 40, 163, 208, 180, 118, 23, 166, 49, 122, 33, 233, 209, 82, 62, 200, 219, 116, 22, 65, 136, 184, 61, 53, 29, 237, 228, 255, 147, 225, 92, 95, 171, 187, 234, 124, 207, 219, 228, 13, 209, 139, 87, 242, 38, 111, 91, 190, 23, 70, 104, 148, 55, 111, 107, 122, 200, 192, 24, 55, 250, 37, 81, 172, 236, 104, 191, 178, 200, 73, 253, 90, 154, 202, 1, 35, 172, 132, 128, 43, 2, 140, 153, 151, 235, 73, 106, 140, 117, 215, 199, 222, 178, 201, 151, 159, 88, 72, 87, 14, 53, 161, 228, 26, 214, 253, 111, 131, 129, 111, 239, 140, 207, 151, 175, 192, 133, 42, 240, 245, 78, 105, 9, 145, 45, 225, 104, 184, 193, 43, 115, 233, 212, 217, 252, 34, 192, 55, 31, 11, 102, 29, 73, 237, 2, 85, 143, 103, 225, 50, 215, 211, 38, 191, 112, 227, 61, 244, 103, 109, 61, 124, 229, 52, 136, 227, 50, 250, 167, 110, 6, 106, 111, 189, 139, 145, 238, 22, 75, 232, 59, 169, 179, 55, 231, 195, 68, 164, 126, 216, 108, 215, 199, 70, 245, 146, 155, 231, 213, 33, 190, 102, 146, 25, 148, 85, 108, 212, 41, 178, 13, 193, 102, 91, 226, 119, 73, 72, 40, 237, 157, 215, 26, 51, 114, 83, 179, 130, 53, 207, 98, 139, 201, 36, 139, 165, 183, 57, 12, 187, 126, 42, 65, 191, 82, 207, 252, 162, 150, 182, 194, 130, 63, 0, 0, 15, 0, 0, 76, 4, 3, 0, 72, 48, 70, 2, 33, 0, 178, 240, 230, 232, 2, 26, 16, 62, 191, 57, 224, 230, 7, 67, 6, 138, 89, 212, 110, 40, 157, 203, 58, 81, 54, 81, 159, 177, 200, 231, 167, 147, 2, 33, 0, 188, 247, 140, 91, 121, 133, 73, 145, 4, 205, 43, 86, 26, 25, 125, 3, 131, 233, 215, 252, 152, 142, 1, 229, 222, 133, 212, 194, 44, 36, 141, 114, 20, 0, 0, 32, 197, 112, 180, 39, 179, 145, 179, 187, 117, 140, 94, 21, 55, 200, 174, 72, 81, 117, 21, 116, 51, 127, 18, 76, 112, 6, 19, 160, 188, 157, 29, 243, 22];

    let iv = [20, 167, 210, 177, 210, 14, 6, 195, 224, 27, 172, 115];

    let plaintext = aes_gcm.open(&[], &iv, &ciphertext, &additional);
    println!("the plaintext is : {:?}", &plaintext);

    assert_eq!(plaintext[..], etalon_result);
}*/