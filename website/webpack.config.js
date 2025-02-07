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
					{ from: './www/shaders/cube.vert', to: './shaders/cube.vert' },
			 		{ from: './www/shaders/cube.frag', to: './shaders/cube.frag' },
					{ from: './www/shaders/teapot.vert', to: './shaders/teapot.vert' },
			 		{ from: './www/shaders/teapot.frag', to: './shaders/teapot.frag' },
					{ from: './www/shaders/cube-tex.vert', to: './shaders/cube-tex.vert' },
			 		{ from: './www/shaders/cube-tex.frag', to: './shaders/cube-tex.frag' },
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
