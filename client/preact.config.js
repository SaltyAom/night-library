import { resolve } from 'path'

import CopyWebpackPlugin from 'copy-webpack-plugin'

const config = (config, env, helpers, options) => {
    config.module.rules[4].use.splice(1, 0, {
        loader: '@teamsupercell/typings-for-css-modules-loader',
        options: {
            banner:
                '// This file is automatically generated from your CSS. Any edits will be overwritten.',
            disableLocalsExport: true
        }
    })

    // Use any `index` file, not just index.js
    config.resolve.alias['preact-cli-entrypoint'] = resolve(
        process.cwd(),
        'src',
        'index'
    )

    // Stylus
    const stylusRule = config.module.rules.find(
        rule => String(rule.test) === String(/\.styl$/)
    )
    stylusRule.use[0].options.options.paths =
        stylusRule.use[0].options.options.paths[0]

    stylusRule.use[0].options.options.module = false

    // config.plugins.push(
    //     new CopyWebpackPlugin({
    //         patterns: [
    //             { from: `${__dirname}/assets`, to: `${__dirname}/build/assets` }
    //         ]
    //     })
    // )
}

export default config
