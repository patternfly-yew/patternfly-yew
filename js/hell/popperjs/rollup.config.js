import { nodeResolve } from '@rollup/plugin-node-resolve';
import replace from '@rollup/plugin-replace';
import { terser } from 'rollup-plugin-terser';

export default {
    input: "main.js",
    output: {
        file: "../../../src/js/popperjs.js",
        format: "esm",
        compact: true,
    },
    plugins: [
        nodeResolve(),
        replace({
            "preventAssignment": true,
            "values": {
                "process.env.NODE_ENV": JSON.stringify('production')
            }
        }),
        terser(),
    ]
}
