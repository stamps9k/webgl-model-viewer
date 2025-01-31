const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  context: path.resolve(__dirname, "./"),
  entry: "./www/js/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "js/bootstrap.js",
  },
 	experiments: {
		asyncWebAssembly: true
	},
	mode: "development",
  devServer: {
    static: {
      directory: path.join(__dirname, './'),
      watch: true
    }
  },
  plugins: 
	[
    new CopyWebpackPlugin
		(
			{
				patterns: 
				[
					{ from: './www/index.html', to: './index.html' },
					{ from: './www/notes.html', to: './notes.html' },
					{ from: './www/shaders/vert-colors.vert', to: './shaders/vert-colors.vert' },
			 		{ from: './www/shaders/vert-colors.frag', to: './shaders/vert-colors.frag' },
					{ from: './www/shaders/texture.vert', to: './shaders/texture.vert' },
			 		{ from: './www/shaders/texture.frag', to: './shaders/texture.frag' },
					{ from: './www/models/teapot.obj', to: './models/teapot.obj' },
					{ from: './www/models/cube.obj', to: './models/cube.obj' },
					{ from: './www/models/cube-tex.obj', to: './models/cube-tex.obj' },
					{ from: './www/textures/cube-tex.tex', to: './textures/cube-tex.tex' },
					{ from: './www/css/bootstrap.min.css', to: './css/bootstrap.min.css' },
					{ from: './www/css/cover.css', to: './css/cover.css' },
					{ from: './www/img', to: './img' }
				]
			}
		)
  ]
};
