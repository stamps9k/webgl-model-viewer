import * as wasm from "wasm-game-of-life";
import * as $ from "../../node_modules/jquery/dist/jquery.min.js"; 
import debug from "debug";

const verbose = debug("app:VERBOSE");
const info = debug("app:INFO");
const error = debug("app:ERROR");

verbose("Verbose debugging enabled.");
info("Info debugging enabled.");
error("Error debugging enabled.");

$.ajaxSetup
(
	{
		beforeSend: function (jqXHR, settings) 
		{
			if (settings.dataType === 'binary')
				settings.xhr = () => $.extend(new window.XMLHttpRequest(), {responseType:'arraybuffer'})
		}
	}
);

$(document).ready(fetch_vert_shader)

function fetch_vert_shader() {
	const url_params = new URLSearchParams(window.location.search);
	var vert_shader = url_params.get('shaders');
	if (vert_shader == null)
	{
		vert_shader = "vert-colors"
	}
	info("Loading shader " + vert_shader + ".vert ...");
	$.ajax
	(
		{
			url: "shaders/" + vert_shader + ".vert",
  			success: function(result) 
			{
				info("... vert shader loaded");
				verbose("Shader text is:");
				verbose(result);
				var resources = new Map();
				resources.set("vert_shader", result);
				fetch_frag_shader(resources)
			},
			error: function(result)
			{
				error("... failed to load vert shader. Error is " + result.status + ": " + result.statusText);
			}
  		}
	);
}

function fetch_frag_shader(resources) {
	const url_params = new URLSearchParams(window.location.search);
	var frag_shader = url_params.get('shaders');
	if (frag_shader == null)
	{
		frag_shader = "vert-colors"
	}
	info("Loading shader " + frag_shader + ".frag ...");
	$.ajax
	(
		{
			url: "shaders/" + frag_shader + ".frag",
			success: function(result)
			{
				info("... frag shader loaded");
				verbose("Shader text is:");
				verbose(result);
				resources.set("frag_shader", result);
				fetch_model(resources);
  			},
			error: function(result)
			{
				error("... failed to load vert shader. Error is " + result.status + ": " + result.statusText);
			}
  		}
	);
}

function fetch_model(resources) {
	const url_params = new URLSearchParams(window.location.search);
	var model = url_params.get('model');
	if (model == null) 
	{
		info("Loading model cube.obj...");
		$.ajax
		(
			{
        		url: "models/cube.obj",
        		success: function(result) 
				{
					info("... model loaded");
					verbose("Model text is:");
					verbose(result);
        			resources.set("cube", result);
					init(resources);
				},
        		error: function(result) 
				{
					error("... failed to fetch model. Error is " + result.status + ": " + result.statusText);
    			}
    		}
		);
	} else {
		var url = "models/" + model + ".obj";
		info("Loading model " + url + "...");
		$.ajax
		(
			{
        		url: url, 
				processData: false,
        		success: function(result) 
				{
					info("... model loaded");
					verbose("Model text is:");
					verbose(result);
        			resources.set("cube", result);
        			if (model.includes("tex"))
					{
						fetch_texture(resources);
					} else {
						init(resources);
					}
				},
        		error: function(result) 
				{
					error("... failed to fetch model. Error is " + result.status + ": " + result.statusText);
    			}
    		}
		);
	}
}

function fetch_texture(resources) {
	const url_params = new URLSearchParams(window.location.search);
	const model = url_params.get('model');
	var url = "textures/" + model + ".tex"; 
	info("Loading texture " + url + "...");
    $.ajax({
        url: url,
		processData: false,
		dataType: "binary",
        success: function(result) {
			info("... texture loaded");
			info("Converting texture to Base64 String...");
			var result_b = new Uint8Array(result);
			const binString = Array.from(result_b, (byte) =>
				String.fromCodePoint(byte),
			).join("");
			var result_b64 = btoa(binString);
			info("... texture stringified.");
			verbose("Full string is:");
			verbose(result_b64);
            resources.set("texture", result_b64);
			init(resources);
        },
        error: function(result) {
            error("... failed to fetch texture. Error is " + result.status + ": " + result.statusText);
        }
    });
}

function init(resources) {
    wasm.initialize_web_gl(resources);
}

