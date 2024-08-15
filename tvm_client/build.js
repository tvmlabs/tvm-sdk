const fs = require('fs');
const os = require('os');
const {
    gz,
    spawnProcess,
    deleteFolderRecursive,
    main,
    version,
    root_path,
    postBuild,
    devMode,
} = require('./build-lib');
const platform = os.platform();
const arch = os.arch();

main(async () => {
    await spawnProcess('cargo', ['build', '--release']);
    deleteFolderRecursive(root_path('bin'));
    fs.mkdirSync(root_path('bin'), { recursive: true });
    const platformNames = {
        linux: {
          x64: [['lib{}.so', '']]
        },
        win32: {
          x64: [['{}.dll.lib', '_lib'], ['{}.dll', '_dll']]
        },
        darwin: {
          x64: [['lib{}.dylib', '']],
          arm64: [['lib{}.dylib', '']]
        },
    };
    for (const [src, dstSuffix] of platformNames[platform][arch] || []) {
        const target = ['..', 'target', 'release', src.replace('{}', 'tvm_client')];
        await postBuild(target, platform);
        await gz(
            target,
            `tonclient_${version}_${platform}${dstSuffix || ''}`, [__dirname, 'build']);
    }
});
