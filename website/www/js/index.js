import * as wasm from "wasm-game-of-life";
import * as $ from "../../node_modules/jquery/dist/jquery.min.js"; 

$.ajaxSetup({
    beforeSend: function (jqXHR, settings) {
      if (settings.dataType === 'binary')
        settings.xhr = () => $.extend(new window.XMLHttpRequest(), {responseType:'arraybuffer'})
    }
  })

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
				processData: false,
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
		processData: false,
		dataType: "binary",
        success: function(result) {
			var result_b = new Uint8Array(result);
			const binString = Array.from(result_b, (byte) =>
				String.fromCodePoint(byte),
			).join("");
			var result_b64 = btoa(binString);
			console.log(result_b64);
            resources.set("texture", result_b64);
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

