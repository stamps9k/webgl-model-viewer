import * as wasm from "wasm-game-of-life";
import * as $ from "../../node_modules/jquery/dist/jquery.min.js"; 

$(document).ready(fetch_vert_shader);
    

function fetch_vert_shader() {
  const url_params = new URLSearchParams(window.location.search);
	var vert_shader = url_params.get('shaders');
	if (vert_shader == null)
	{
		vert_shader = "vert-colors"
	}
	$.ajax
	(
		{
			url: "shaders/" + vert_shader + ".vert",
  		success: function(result) 
			{
  			var resources = new Map();
				resources.set("vert_shader", result);
				console.log("Vert shader loaded...");
				fetch_frag_shader(resources)
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
	$.ajax
	(
		{
			url: "shaders/" + frag_shader + ".frag",
			success: function(result)
			{
				resources.set("frag_shader", result);
				console.log("Frag shader loaded...");
				fetch_model(resources);
  		}
  	}
	);
}

function fetch_model(resources) {
	const url_params = new URLSearchParams(window.location.search);
	var model = url_params.get('model');
	if (model == null) 
	{
		$.ajax
		(
			{
        url: "models/cube.obj",
        success: function(result) 
				{
        	resources.set("cube", result);
        	console.log("cube loaded...");
					init(resources);
				},
        error: function(result) 
				{
					console.log("Model fetched failed.")
    		}
    	}
		);
	} else {
		$.ajax
		(
			{
        url: "models/" + model + ".obj",
        success: function(result) 
				{
        	resources.set("cube", result);
        	console.log(model + " loaded...");
        	if (model.includes("tex"))
					{
						fetch_texture(resources);
					} else {
						init(resources);
					}
				},
        error: function(result) 
				{
     			console.log("Model fetched failed.")
    		}
    	}
		);
	}
}

function fetch_texture(resources) {
		const url_params = new URLSearchParams(window.location.search);
		const model = url_params.get('model');
    $.ajax({
        url: "textures/" + model + ".tex",
        success: function(result) {
            resources.set("texture", result);
            console.log("texture loaded...");
						init(resources);
        },
        error: function(result) {
            console.log("Model fetched failed.")
        }
    });
}

function init(resources) {
    wasm.initialize_web_gl(resources);
}

