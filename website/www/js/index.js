import * as wasm from "wasm-game-of-life";
import * as $ from "../../node_modules/jquery/dist/jquery.min.js"; 

$(document).ready(fetch_vert_shader);
    

function fetch_vert_shader() {
    $.ajax({
        url: "shaders/standard.vert",
        success: function(result) {
            var resources = new Map();
            resources.set("vert_shader", result);
            console.log("Vert shader loaded...");
            fetch_frag_shader(resources)
        }
    });
}

function fetch_frag_shader(resources) {
    $.ajax({
        url: "shaders/standard.frag",
        success: function(result) {
            resources.set("frag_shader", result);
            console.log("Frag shader loaded...");
            fetch_model(resources);
        }
    });
}

function fetch_model(resources) {
		const url_params = new URLSearchParams(window.location.search);
		const model = url_params.get('model');
    $.ajax({
        url: "models/" + model + ".obj",
        success: function(result) {
            resources.set("cube", result);
            console.log("cube loaded...");
            if (model.includes("tex"))
						{
							fetch_texture(resources);
						} else {
							init(resources);
						}
        },
        error: function(result) {
            console.log("Model fetched failed.")
        }
    });
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

