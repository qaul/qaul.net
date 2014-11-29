var links, nodes, width, height, force, svg, path, circle, text;

// load nodes
function init_network()
{
	$.ajax({
		url:   "gettopology.json",
		cache: false, // needed for IE
		dataType: "json",
		success: function(data) {
			if(data.available)
				draw_network(data);
			else
			{
				// data is not yet available schedule receive for later
				setTimeout(function(){init_network();}, 500);
			}
		}
	});
}

function draw_network(data)
{
	links = data.links;
	nodes = data.nodes;
	
	// search nodes from links
	// calculate link quality
	links.forEach(function(link) {
	  link.source = nodes[link.source] || (nodes[link.source] = {name: link.source, type: "ip"});
	  link.target = nodes[link.target] || (nodes[link.target] = {name: link.target, type: "ip"});
	  
	  if(link.lq < 2)
	  	link.type = "good";
	  else if(link.lq < 5)
	  	link.type = "ok";
	  else
	  	link.type = "bad";
	});

	width = $(window).width();
	height = $(window).height();

	force = d3.layout.force()
		.nodes(d3.values(nodes))
		.links(links)
		.size([width, height])
		.linkDistance(100)
		.charge(-300)
		.on("tick", tick)
		.start();

	svg = d3.select("body").append("svg")
		.attr("width", width)
		.attr("height", height);

	// update drawing area, when window size changes
	$(window).resize(function() {
	  width = $(window).width();
	  height = $(window).height();
	  force.size([width, height]);
	  svg.attr("width", width)
		 .attr("height", height);
	});

	// Per-type markers, as they don't inherit styles.
	svg.append("defs").selectAll("marker")
		.data(["good", "ok", "bad"])
	  .enter().append("marker")
		.attr("id", function(d) { return d; })
		.attr("viewBox", "0 -5 10 10")
		.attr("refX", 15)
		.attr("refY", -1.5)
		.attr("markerWidth", 6)
		.attr("markerHeight", 6)
		.attr("orient", "auto")
	  .append("path")
		.attr("d", "M0,-5L10,0L0,5");

	path = svg.append("g").selectAll("path")
		.data(force.links())
	  .enter().append("path")
		.attr("class", function(d) { return "link " + d.type; })
		.attr("marker-end", function(d) { return "url(#" + d.type + ")"; });

	circle = svg.append("g").selectAll("circle")
		.data(force.nodes())
	  .enter().append("circle")
		.attr("r", 10)
		.call(force.drag);

	text = svg.append("g").selectAll("text")
		.data(force.nodes())
	  .enter().append("text")
		.attr("x", 15)
		.attr("y", ".31em")
		.attr("class", function(d){ return d.type; })
		.text(function(d) { return d.name; });
}

// Use elliptical arc path segments to doubly-encode directionality.
function tick() {
  path.attr("d", linkArc);
  circle.attr("transform", transform);
  text.attr("transform", transform);
}

function linkArc(d) {
  var dx = d.target.x - d.source.x,
      dy = d.target.y - d.source.y,
      dr = Math.sqrt(dx * dx + dy * dy);
  return "M" + d.source.x + "," + d.source.y + "A" + dr + "," + dr + " 0 0,1 " + d.target.x + "," + d.target.y;
}

function transform(d) {
  return "translate(" + d.x + "," + d.y + ")";
}

// ---------------------------------------------

$(init_network);
