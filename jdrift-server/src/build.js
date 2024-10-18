import esbuild from 'esbuild';

const args = process.argv.slice(2);
const watchMode = args.includes('--watch');


if (watchMode) console.log("Watching");

esbuild.context({
    entryPoints: ['src/main.ts'],
    bundle: true,
    platform: 'browser',
    minify: true,
    sourcemap: true,
    outfile: 'src/build/main.js',
    tsconfig: 'tsconfig.json',
    target: 'es2020',
//    watch: watchMode,
}).then(async (ctx) => {
    console.log('Build complete');
    if (watchMode) await ctx.watch();
}).catch((error) => console.error(error));
