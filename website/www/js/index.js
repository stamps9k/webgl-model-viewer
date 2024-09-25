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
    $.ajax({
        url: "models/teapot.obj",
        success: function(result) {
            resources.set("teapot", result);
            console.log("teapot loaded...");
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

