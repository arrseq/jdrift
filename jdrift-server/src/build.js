import esbuild from 'esbuild';

const args = process.argv.slice(2);
const watchMode = args.includes('--watch');

esbuild.build({
    entryPoints: ['src/main.ts'],
    bundle: true,
    platform: 'browser',
    minify: true,
    sourcemap: true,
    outfile: 'src/build/main.html',
    tsconfig: 'tsconfig.json',
    target: 'es2020',
//    watch: watchMode,
}).then(async (ctx) => {
    console.log('Build complete');
    if (watchMode) await ctx.watch();
}).catch(() => process.exit(1));
