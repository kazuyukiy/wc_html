'use strict';

let page_json;

function bodyOnload () {

	page_json = page_json_read();

	const elePageTarget = document.createElement('div');
	document.body.appendChild(elePageTarget);
	
	let page = new Page(elePageTarget, page_json);
	page.eleDraw();

	page.editor().mode_off();
	
	// let eleA = document.querySelector("a[href=\"##abc$\"]");
	// let eleA = document.querySelector("a[href=\"#keymap_us\"]");

	// bodyOnloadFetch();
	// fetchDataTest01();

	scrollUrlHash();

	// test02();

} // end of function bodyOnload;

// Get page string data from <span id="page_json_str">{...data...}</span>
// in HTML source .
// Return as javascript value .
function page_json_read () {
	// console.log("wc.js function page_json_read");

	let ele_page_json_str = document.getElementById("page_json_str");
	if(! ele_page_json_str){ return; }
	let page_json_str = ele_page_json_str.innerHTML;

	page_json_str = entityReferenceUnset(page_json_str);

	// Convert page_json_str (string) to javascript value .
	// And return it .
	// text \" becomes to value " .
	let f0 = new Function("return " + page_json_str + ";");
	return f0();
	
} // end of function page_json_read

function entityReferenceSet(str) {

	str = str.replace(/[<>&]/g, function(){
		let ref = ENTITY_REFERENCE[arguments[0]];
		
		if(ref){ return ref;}
		return arguments[0];
	});

	return str;

} // end of function entityReferenceSet

const ENTITY_REFERENCE = {
	'<' : '&lt;'
	, '>' : '&gt;'
	, '&' : '&amp;'
	// , '' : ''
};

// 
const REFERENCE_ENTITY = {
	'&lt;' : '<'
	, '&gt;' : '>'
	, '&amp;' : '&'
	// , '' : ''
};

// Replace charactors that can not be used in HTML and escaped .
//	'&lt;' : '<'
//	, '&gt;' : '>'
//	, '&amp;' : '&'
// Therefore \" must not replace to " since it still in text json data .
// \" will be handled as value '"' when put it into a javascript variable.
function entityReferenceUnset(str) {

	let re = /&[^;]+;/g;

	str = str.replace(
		//		return str.replace()
		re,
		function() {
			let entity = REFERENCE_ENTITY[arguments[0]];
			if(entity){ return entity;}
			return arguments[0];
		}
	);

	return str;

} // end of function entityReferenceUnset

class Page {
	// console.log("wc.js class Page" );

	// constructor(parentEle, page_json) {}
	// eleTarget is place to put elements .
	// eleTarget should not be delited,
	// Because it may be pathed to sub class that shoul be kept as the target .
	constructor(eleTarget, page_json) {
		this.page = this;
		this.eleTarget(eleTarget);
		this.page_json = page_json;
	} // end of constructor

	// target to where this.ele() put .
	eleTarget() {
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

		let ele = document.createElement('div');

		return this.eleTarget(ele);
		
		// return this.eleTargetObj;
	} // end of eleTarget

	// Refresh drawing .
	eleDraw() {
		// console.log("wc.js class Page eleDraw()" );
		this.editor().eleDraw();
		this.navi().eleDraw();
		this.subsectionTop().childrenIndex().eleDraw();
		this.subsectionTop().childrenEleDraw();

		// console.log("wc.js class Page eleDraw() oneChildAtleast" );

		this.subsectionTop().oneChildAtleast();

		
	} // end of eleDraw

	// Create targets for children .
	// Return hash data of the targets .
	childEleTargets() {
		if(0 < arguments.length){
			this.childEleTargetsObj = arguments[0];
			return this.childEleTargetsObj;
		}
		if(this.childEleTargetsObj){ return this.childEleTargetsObj; }

		let h = {};
		for(name of [
			// "top"
			"editor"
			, "navi"
			, "index"
			, "subsection"
		]){

			// Set an eleTarget to this.eleTarget() .
			const eleTarget = document.createElement('div');
			this.eleTarget().appendChild(eleTarget);

			h[name] = eleTarget;

		}
		
		return this.childEleTargets(h);

	} // end of childEleTargets

	editor() {
		if(! this.editorObj){
			const target = this.childEleTargets()["editor"]
			this.editorObj = new Editor(this.page, target);
		}

		return this.editorObj;

	} // end of editor
	
	navi() {
		if(! this.naviObj){
			const target = this.childEleTargets()["navi"]
			this.naviObj = new Navi(this.page, target);
		}

		return this.naviObj;

	} // end of navi

	// useage
	// let subsection = this.subsection(id);
	//
	// to delete
	// this.subsection(id, undefined)
	subsection(id) {
		// console.log("wc.js class Page subsection()" );

		if(this.subsectionObj == undefined){ this.subsectionObj = {}; }
		if(id == undefined){ return; }
		if(this.subsectionObj[id]){
			if(1 < arguments.length && arguments[1] == undefined){
				delete this.subsectionObj[id];
			}
			return this.subsectionObj[id];
		}

		// return undef if no data exists .
		let data = this.page_json["data"]["subsection"]["data"][id];
		if(data == undefined){ return; }
		
		const target = this.childEleTargets()["subsection"]
		this.subsectionObj[id] = new Subsection(this.page, id);
		this.subsectionObj[id].eleTarget(target);

		return this.subsection(id);

	} // end of subsection

	subsectionTop() {

		if(this.subsectionTopObj == undefined){
			let top = this.subsection(0);
			// only subsection with its id 0 has this target for childrenIndex .
			// Other subsections has a target of childIndex on
			// childIndexItem of the parent .
			const target = this.childEleTargets()["index"]
			top.childrenIndex().eleTarget(target);
			this.subsectionTopObj = top;
		}

		return this.subsectionTopObj;
		
	} // end of subsectionTop

	id_new() {
		let id_data = this.page_json["data"]["subsection"]["id"];
		
		if(0 < id_data["id_notinuse"].length){
			return id_data["id_notinuse"].shift();
		}
		
		let id_new = id_data["id_next"];
		id_data["id_next"] = id_new + 1;
		return id_new;
		
	} // end of id_new

	// Recieve id of subsection deleted
	// and put back to id_notinuse
	id_return(id) {
		if(id == undefined){ return; }

		this.page_json["data"]["subsection"]["id"]["id_notinuse"].push(id);
		
	} // end of id_return

	subsectionNew() {
		// console.log("wc.js class Page subsectionNew()" );

		let id = this.id_new();
		this.page.page_json
		
		// console.log("wc.js class Page subsectionNew() id:"  + id);

		// data["child"] = [];
		const subsectionsData = this.page.page_json["data"]["subsection"]["data"];
		subsectionsData[id] = {"content" : [], "child" : []};
		// "parent" : "",
		// "id" : "0",
		// "title" : "",
		// "href" : "",
		// "content" : [],
		// "child" : [ 1 ]
		
		return this.subsection(id);

	} // end of subsectionNew

	// Set all <a href="..."> elements adding eventListener to fetch
	// asking the server if the href is valid. 
	hrefEventAdd() {
		// console.log("wc.js class Page hrefEventAdd()" );

		let as = this.eleTarget().querySelectorAll("a");
		as.forEach(function(a){
			let href = a.getAttribute("href");
			if(href){
				a.addEventListener("click", hrefEventHandle);
			}
		});
		
	} // end of hrefEventAdd

	hrefEventRemove() {
		// console.log("wc.js class Page hrefEventRemove()" );

		let as = this.eleTarget().querySelectorAll("a");
		as.forEach(function(a){
			let href = a.getAttribute("href");
			if(href){
				a.removeEventListener("click", hrefEventHandle);
			}
		});
		
	} // end of hrefEventRemove

	href_reference() {

		if(this.href_reference_obj == undefined){
			this.href_reference_obj = new HrefReference(this.page);
		}

		return this.href_reference_obj;
		
	} // end of href_reference

	page_move() {

		if(this.page_move_obj == undefined){
			this.page_move_obj = new PageMove(this.page);
		}

		return this.page_move_obj;
		
	} // end of page_move

} // end of class Page

// obj.ele : node to draw
// obj.ele(undefined) : remove obj.drawn
// obj.targetFcode : code to get the target .
// obj.drawn : current node drawn
// obj.nextFcode : code to get next obj where insert before.
//
// useage
//	// See function eleHandle()
//	editorEle(...args) {
//
//		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
//		return eleHandle(this, this.editorEleObj, ...args);
//
//	} // end of editorEle
//
function eleHandle(that, obj, ...args) {
	// console.log("wc.js function eleHandle" );

	// DBG
	// let dbg;
	// if(that.constructor.name == ""){ dbg = true; }
	// if(dbg){
		// console.log("wc.js function eleHandle " );
	// }

	if(args[0] == undefined){ return obj; }
	
	let def = args[0];

	// Apply args[0] value to obj
	for(let key in def){
		if(def[key] == undefined){
			delete obj[key];
			continue;
		}
		obj[key] = def[key];
	}

	// args[0] has a key "ele"
	if(Object.keys(def).includes("ele")){
		let eleDrawn = obj.drawn;
		// Case ele (in args[0]) is undefined
		// Close (remove) eleDrawn .
		if(def.ele == undefined){
			if(eleDrawn){
				// eleDrawn exists
				// delet eleDrawn
				if(eleDrawn.parentNode.contains(eleDrawn)){
					eleDrawn.parentNode.removeChild(eleDrawn);
					delete obj.drawn;
				}
			}
		}
		// Draw obj.ele 
		// Replace if it was drawn .
		else{

			// Consider to replace current ele to new ele
			// but current ele is not in a new target,
			// current ele might be in the previous target
			
			const eleNew = obj.ele;
			let doDraw = true;
			// insert before next
			// if obj.nextFcode is defineded
			if(obj.nextFcode){

				const fcode = Function("return this." + obj.nextFcode + ";");
				const fref = fcode.apply(that);
				if(fref){
					const eleNext = fref.apply(that);
					if(eleNext){
						// Remove eleDrawn
						if(eleDrawn){
							eleDrawn.parentNode.removeChild(eleDrawn);

							
						}
						eleNext.parentNode.insertBefore(eleNew, eleNext);
						// Drawn, no need to draw any more
						doDraw = false;
					}
				}
			}
			// replace eleDraw with eleNew
			// else if(eleDrawn){
			//
			// Replace
			if(doDraw && eleDrawn){
				eleDrawn.parentNode.replaceChild(eleNew, eleDrawn);
				doDraw == false;
			}
			// draw eleNew
			// else {
			if(doDraw){
				const fcode = Function("return this."+ obj.targetFcode +";");
				const fref = fcode.apply(that);
				if(fref){
					const eleTarget = fref.apply(that);
					eleTarget.appendChild(eleNew);
				}
			}
			obj.drawn = eleNew;
			delete obj.ele;
		}
	}

	return obj;

} // end of function eleHandle

function hrefEventHandle(event) {
	// console.log("wc.js function hrefEventHandle ");

	let href = event.target.getAttribute("href");

	// href : #abc
	// move to #abc .
	// #: move to top
	if(href == "#"){
		window.scrollTo(0, 0);
		return;
	}
	
	if(href.match(/^#(.+)/)){

		// location.href = href;
		// remove #
		scrollHash(href.slice(1));

		return;
	}

	if(href == "javascript:history.back()"){
		javascript:history.back();
		return;
	}

	let data = {"href" : href};
	
	console.log("wc.jp function hrefEventHandle post href:" + href)
	let res = posData("href", data);

	res.then(data => {
		// console.log("wc.jp function hrefEventHandle res.data:" + data.filename);
		console.log("wc.jp function hrefEventHandle res.path:" + data.path);
		console.log("wc.jp function hrefEventHandle res.dest:" + data.dest);

		if(data.dest){
			location.href = data.dest;
		}
		
	});

	preventContextmenu(event); // prevent move to href2;

} // end of function hrefEventHandle

class Temp {
	// console.log("wc.js class Temp");

	constructor(page, eleTarget) {
		this.page = page;
		this.eleTarget(eleTarget);
	} // end of constructor

	eleTarget() {
		
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

		// let ele;
		// return this.eleTarget(ele);

	} // end of eleTarget

	// The next element where this.ele() is appended before .
	// If this.ele() is to be the last of this.eleTarget().children ,
	// return undefined of remove this.eleNext code .
	// eleNext() {
	// } // end of eleNext
	
	eleDraw() {

		let ele = this.ele_html();
		
		let eleDef = {
			"ele" : ele,
			"targetFcode" : "eleTarget",
			"nextFcode" : "eleNext",
		};

		this.ele(eleDef);

	} // end of eleDraw

	// See function eleHandle()
	ele(...args) {

		if(this.eleObj == undefined){ this.eleObj = {}; }
		return eleHandle(this, this.eleObj, ...args);

	} // end of ele

	// See function eleHandle()
	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

} // end of class Temp

class Editor {
	// console.log("wc.js class Editor");

	constructor(page, eleTarget) {
		this.page = page;
		this.eleTarget(eleTarget);
	} // end of constructor

	eleTarget() {
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

		// let ele;
		// return this.eleTarget(ele);

	} // end of eleTarget

	eleDraw() {

		this.menu().eleDraw();
		
	} // end of eleDraw

	ele() {
		
	} // end of ele

	menu() {
		if(this.menuObj == undefined){
			this.menuObj = new EditMenu(this.page, this.eleTarget());
		}
		return this.menuObj;
		
	} // end of menu

	// set editor mode off
	// also set mode_on eventListener
	mode_off() {

		// remove eventListener to open editor
		if(this.open_f){
			document.body.removeEventListener('mouseup', this.open_f);
		}

		// set edit mode switch on the last item of navi
		// let navi_last_ele = this.page.navi().navi_item_last().ele();
		let lastEle = this.page.navi().itemLast().ele().drawn;
		this.mode_on_f = this.mode_on.bind(this);
		lastEle.addEventListener('mouseup', this.mode_on_f);
			
		this.page.hrefEventAdd();
		
		// event.stopPropagation(); // prevent editorOpen;
		
	} // end of mode_off

	// set editor mode on
	mode_on(event) {

		this.page.editor().menu().open();

		// set eventListener on body to watch requests to edit each item
		// watch events on body.
		// when body gets an event, it will figure out what item was selected
		// in the body at Editor.this.object_to_open()
		this.open_f = this.open.bind(this);
		document.body.addEventListener('mouseup', this.open_f);

		// prevent mouse right;
		// document.body.addEventListener('contextmenu', preventContextmenu);

		// remove eventListenter from navi last
		let lastEle = this.page.navi().itemLast().ele().drawn;
		lastEle.removeEventListener('mouseup', this.mode_on_f);

		this.page.hrefEventRemove();

		event.stopPropagation(); // prevent editorOpen;

	} // end of mode_on

	// edtor open
	// this open is called when body is clidked on rught
	// the eventListener was set at this.mode_on()
	// eventuary editor_open() is called for an object clicked at this.editor_open() 
	open(event) {
		// console.log("wc.js class Editor open");

		if(event.button != 2){ return;} // click right;

		let object_to_open = this.object_to_open(event);
		if(object_to_open == undefined){ return;}

		this.editor_open(object_to_open);

	} // end of open
	
	editor_open(object_to_open) {
		// console.log("wc.js class Editor editor_open");
		
		if(this.current_object()){
			// opening the current class
			// ignore the request
			if(this.current_object() == object_to_open){ return; }			

			this.editor_close();
		}

		this.current_object(object_to_open);
		this.current_object().editor_open();
		
	} // end of editor_open

	current_object() {
		if(0 < arguments.length){
			this.current_object_obj = arguments[0];
		}
		return this.current_object_obj;
	} // end of current_object

	// Find out whta object to be open from event .
	object_to_open(event) {
		// console.log("wc.js class Editor object_to_open()");
		
		let target = target2(event.target);
		if( ! target){ return;}

		let class_name = target[0];
		let class_ele = target[1];
		const json_id = class_ele.getAttribute('data-json_id');

		let objToOpen;
		if(class_name == "naviItem"){
			objToOpen = this.page.navi().items()[json_id];
		}
		
		if(class_name == "subsectionListItem"){
			let subsection = this.page.subsection(json_id);
			objToOpen = subsection.indexItem();
		}
		
		if(class_name == "subsection"){
			objToOpen = this.page.subsection(json_id);
		}

		if(class_name == "subsectionContent"){
			let subsection = this.page.subsection(json_id);
			for(let content of subsection.contents()){
				if(content.ele().drawn == class_ele){
					objToOpen = content;
					break;
				}
			}
			if(subsection.contents().length == 0){
				objToOpen = subsection.contentBlank();
			}
		}

		return objToOpen;

	} // end of object_to_open

	// convert html to node elements .
	// also set eventListener for each input button
	// if editor_menu_item exists in this.xxx() set it as eventListener action
	// if editor_menu_item exists in ob.xxx() set it as eventListener action
	ele_setup(obj, html) {
		// console.log("wc.js class Editor ele_setup()");
		
		if(arguments[0] == undefined){ return undefined;}

		html = html.trim();

		const eleDiv = document.createElement('div');
		eleDiv.innerHTML = html;

		let ele;
		// to make ele has only one element at the top layer
		// if number of elements in top leyer is one, make it as ele;
		// if several top elements in html, make ele is eleDiv
		if(eleDiv.childNodes.length == 1){
			ele = eleDiv.childNodes[0];
		}else{ ele = eleDiv; }

		// add eventLisnter to buttons
		// to call this.<'editor_req_'+name>(obj);
		// ex.: this.editor_req_cancel(obj);
		for(const name of editor_menu_item){
			const eleSw = ele.querySelector(".editor_"+name);
			if(eleSw == undefined){ continue;}

			// search the function of this
			const f0_edi_code = new Function('return this.editor_'+name+';');
			const f0_edi = f0_edi_code.apply(this);
			if(f0_edi){
				// method this.editor_name0 exists in class Editor
				//
				// if use "this" instead of this_editor,
				// "this" will be the event.target, / may be a button element
				let this_editor = this;
				eleSw.addEventListener('click', function(event){
					f0_edi.apply(this_editor, [obj, event])
				} );
				continue;
			}

			const f0_obj_code = new Function('return this.editor_'+name+';');
			const f0_obj = f0_obj_code.apply(obj);
			if(f0_obj){
				// obj.editor_name0 exists
				eleSw.addEventListener('click', function(event){
					f0_obj.apply(obj, [event])
				} );
				continue;
			}
			
		}
		
		return ele;	

	} // end of ele_setup

	// actions for editor input buttons
	// this.editor_cancel() => this.editor_close() => obj.editorClose()
	// this.editor_close() -> obj.editor_close();
	// this.editor_enter() => obj.editor_enter()
	// others  editor_xxx => obj.editor_xxx()

	editor_cancel() {
		// console.log("wc.js class Editor editor_cancel()");

		this.editor_close();

		event.stopPropagation(); // prevent further process
		
	} // end of editor_cancel

	editor_close() {

		let obj = this.current_object();
		if(obj == undefined){ return;}

		if(obj.editor_close){ obj.editor_close(); }
		
		// close the editor 
		obj.editorEle({"ele" : undefined});
		
		this.current_object(undefined);
		
	} // end of editor_close

	editor_enter(obj) {

		obj.editor_enter();
		// obj.editorEnter();
		
	} // end of editor_enter

	// This is to make inside of obj.editlr_enter() at this.editor_enter() easy
	// to call back this.menu.changed() .
	editor_changed() {
		if(0 < arguments.length){
			return this.menu().changed(...arguments);
		}
		return this.menu().changed();

	} // end of editor_changed

} // end of class Editor

const editor_menu_item  = [
	'cancel', 'enter', "insertMenu", "moveMenu"// , 'insertBefore', "insertAfter"
	, 'newPage', 'subContent'
	, 'delete'
	, 'deleteExecute'
	, 'deleteCancel'
];

const classEditList2 =
	  [
		  // 'navi',
		  'subsectionListItem',
		  'subsection',
		  'subsectionContent'
		  , 'naviItem'
		  // ,'listItem',  'index'
	  ]; // end of const classEditList2 

function editorClassMatch2 (elePart) {
	// console.log("wc.js function editorClassMatch2");
	const classList = elePart.classList;

	if(classList == undefined){ return;}
	for(let i=0; i < classEditList2.length; i++){
	    if(classList.contains(classEditList2[i])){
			return classEditList2[i];
	    }
	}
	return;
} // end of function editorClassMatch2

function target2 (elePartArg) {
	// 
	let elePart = elePartArg;
	let className;
	while(elePart){
	    className = editorClassMatch2(elePart);
	    if(className){ break;}
	    elePart = elePart.parentNode;
	}

	if( ! elePart){ return;}

	return [className, elePart];
	
} // end of function target2;

class EditMenu {

	constructor(page, eleTarget) {
		this.page = page;
		this.eleTarget(eleTarget);
	} // end of constructor

	eleTarget() {
		// console.log("wc.js class EditMenu eleTarget() arguments[0]:" + arguments[0]);
		
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

	} // end of eleTarget

	eleDraw() {
		// console.log("wc.js class EditMenu eleDraw()");
		
		// <div id="edit_menu"></div>
		const ele = document.createElement('div');
		ele.setAttribute('id', "edit_menu");

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "eleTarget",
		};
		
		this.ele(eleDef);

	} // end of eleDraw
	
	// See function eleHandle()
	ele(...args) {

		if(this.eleObj == undefined){ this.eleObj = {}; }
		return eleHandle(this, this.eleObj, ...args);

	} // end of ele

	open() {
		// console.log("wc.js class EditMenu open()");

		const ele = document.createElement('div');
		ele.innerHTML = htmlEditMenu;

		// group_top set/unset
		if (this.page.page_json["data"]["page"]["group_top"]) {
			ele().querySelector(".editMenu_group_top_set").value = "Unset Group Top";
		}

		// eventListenerSet
		// editMenu + name
		for(name of editMenuKeys){
			// const eleSw = this.ele().querySelector(".editMenu_"+name);
			const eleSw = ele.querySelector(".editMenu_"+name);
			if(eleSw == undefined){ continue;}
			
			const f0_ref = new Function('return this.'+name+';');
			const f0_edi = f0_ref.apply(this);
			if(f0_edi == undefined){ continue;}
			// if use this instead of this_editor,
			// this will be the event.target, / may be a button element
			let this_editor = this;
			eleSw.addEventListener('click', function(event){
				f0_edi.apply(this_editor, [])
			} );
			
		}

		// ( old form <input type="file" value="Import" class="editModeImport"> )
		
		eleVisibleSet(
			// this.ele(),
			ele,
			{
				'editMenu_exit' : 1
				,'editMenu_saveMenu' : 0
				,'editMenu_exitConfirm' : 0
			}
			// req
		);

		const eleDef = {
			"ele" : ele,
			"targetFcode" : "editorTarget",
		};
		this.editorEle(eleDef);

	} // end of open

	editorTarget() {
		return this.ele().drawn;
	} // end of editorTarget

	// See function eleHandle()
	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

	changed() {
		// this.changedV  true : changed / false : not changed
		
		if(0 < arguments.length){

			// argumetns is changed(ture)
			if(arguments[0]){
				this.changedV = true;

				eleVisibleSet(
					// this.ele(),
					this.ele().drawn,
					{
						'editMenu_exit' : 1
						,'editMenu_saveMenu' : 1
						,'editMenu_exitConfirm' : 0
					}
				);
				
			} else {
				this.changedV = false;
			}
		}
		
		return this.changedV;
		
	} // end of changed

	// Send a request to save the page with data page_json
	save() {
		// console.log("wc.js class EditMenu save:" );
		// req_fetch("json_save", this.page.page_json);
		let res = posData("json_save", this.page.page_json);
		res.then(data => {
			console.log("wc.jp function save res:" + data.res);
		});
		
		
		this.changed(false);
		this.exit();

	} // end of save

	exit() {
		// console.log("wc.js class EditMenu editMenu_exit:" );

		// Confirm discard edited data
		if(this.changed()){
			eleVisibleSet(
				// this.ele(),
				this.ele().drawn,
				{
					'editMenu_exit' : 0
					,'editMenu_saveMenu' : 0
					,'editMenu_exitConfirm' : 1

					,'editMenu_href_reference' : 0
					,'editMenu_group_top_set' : 0
					,'editMenu_page_move_open' : 0
					,'editMenu_page_json_open' : 0
				}
			);
			return;
			
		}

		this.page.editor().editor_close();

		this.editorEle({"ele" : undefined});

		this.page.editor().mode_off();
		
	} // end of exit

	discard() {
		// console.log("wc.js class EditMenu discard:" );

		this.changed(false);
		this.exit();

	} // end of discard

	exitCancel() {
		// console.log("wc.js class EditMenu exitCancel:" );

		// this.changed(true);

		eleVisibleSet(
			// this.ele(),
			this.ele().drawn,
			{
				'editMenu_exit' : 1
				,'editMenu_saveMenu' : 1
				,'editMenu_exitConfirm' : 0

				,'editMenu_href_reference' : 1
				,'editMenu_group_top_set' : 1
				,'editMenu_page_move_open' : 1
				,'editMenu_page_json_open' : 1
				
			}
		);
		

	} // end of exitCancel

	page_move_open() {
		// console.log("wc.js class EditMenu page_move_open" );

		const editor_page_json = new EditorPageMove(this.page);
		this.page.editor().editor_open(editor_page_json);
		
	} // end of page_move_open

	page_json_open() {
		// console.log("wc.js class EditMenu page_json_open" );

		const editor_page_json = new EditorPageJson(this.page);
		this.page.editor().editor_open(editor_page_json);
		
	} // end of page_json_open

	group_top_set() {
		this.page.page_json["data"]["page"]["group_top"] = true;
		this.changed(true);
	} // end of group_top_set

	href_reference() {
		// console.log("wc.js class EditMenu href_reference" );

		this.page.editor().editor_open(this.page.href_reference());

		// page.editor() // Editor
	} // end of href_reference

} // end of class EditMenu

const htmlEditMenu = `
    <table class="editModeTable">
      <tr>
	<td>
	  Edit MODE 
	  <input type="button" value="Exit" class="editMenu_exit">

	  <span class="editMenu_saveMenu">
	   /
	   <input type="button" value="Save" class="editMenu_save">
	  </span>

	  <span class="editMenu_exitConfirm">
	  Exit without saving ?
	  <input type="button" value="Discard changes" class="editMenu_discard">
	  <input type="button" value="Cancel" class="editMenu_exitCancel">
	  </span>

	  <input type="button" value="href_reference" class="editMenu_href_reference">
	  <input type="button" value="page_move" class="editMenu_page_move_open">
	  <input type="button" value="page_json" class="editMenu_page_json_open">


	  <input type="button" value="Set Group Top" class="editMenu_group_top_set">


	</td>
      </tr>
    </table>
`; // end of const htmlEditMenu

// editMenuKeys : editMenu + key
// 'editMenu_exit'
// 'editMenu_save'
// 'editMenu_discard'
// 'editMenu_exitCancel'
// 'editModeImport'
//
const editMenuKeys = [
	'exit'
	, 'save'
	, 'exit'
	, 'discard'
	, 'exitCancel'
	// , 'editModeImport'
	, 'page_move_open'
	, 'page_json_open'
	, 'group_top_set'
	, 'href_reference'
]; // end of const editMenuKeys

function editorMoveEventListenerSet(caller, classList) {

		for(let target of classList){
			let eleTarget = target.ele().drawn;
			const f0_ref = new Function('return this.editor_move;');
			const f0 = f0_ref.apply(caller);
			// let caller = this;
			target.editorMoveOn = {
				trigger: "click",
				action: function (event){
					f0.apply(caller, [target, event]);
				},
			};
			eleTarget.addEventListener(
				target.editorMoveOn.trigger,							
				target.editorMoveOn.action,
			);
		}
		
} // end of function editorMoveEventListenerSet
	
class Navi {
	// console.log("wc.js class Navi");

	constructor(page, eleTarget) {
		this.page = page;
		this.eleTarget(eleTarget);
	} // end of constructor

	items() {
		// console.log("wc.js class Navi items()");
		if(0 < arguments.length){
			this.eleItemsObj = arguments[0];

			if(this.eleItemsObj == undefined){ return; }
						
			return this.eleItemsObj;
		}

		if(this.eleItemsObj){ return this.eleItemsObj; }

		let items = [];

		let page = this.page;
		// let eleTarget = this.ele();
		let eleTarget = this.ele().drawn;
		let navi = this;
		page.page_json["data"]["navi"].forEach(
			function (data) {
				// item, index, array
				items.push(new NaviItem(page, eleTarget, navi, data));
			}
		)
		
		return this.items(items);
		
	} // end of items

	itemNew(target) {
		// console.log("wc.js class Navi itemNew()");
		
		// target: target of NaviItem to put a new NaviItem aftar that.
		let navis = this.page.page_json.data.navi;
		// if navis == [], put new NaviItem to index 0 .
		let index = 0;
		for(index = 0;  index < navis.length; index++){
			if(target.index() == index){
				// Insert new NaviItem data after target.index 
				index += 1;
				break;
			}
		}

		navis.splice(index, 0, []);

		this.items(undefined);

		return this.items()[index];

	} // end of itemNew

	// find item from this.items
	// Return the index number of item
	itemIndex(item) {
		// item: NaviItem
		for(let i = 0; i < this.items().length; i++){
			// same data
			if(item.data() == this.page.page_json["data"]["navi"][i]){
				return i;
			}
		}

	} // itemIndex 

	itemLast() {
		
		return this.items().slice(-1)[0];
		
		// let items = this.items();
		// const indexLast = items.length -1;
		// return items[indexLast];
	} // end of itemLast

	// Return item that come after itemArg
	itemNext(itemArg) {

		let match = false;
		for(const item of this.items()){
			if(item == itemArg){
				match = true;
				continue;
			}
			if(match){ return item; }
		}

	} // end of itemNext

	eleTarget() {
		
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

		// let ele;
		// return this.eleTarget(ele);
		
	} // end of eleTarget

	eleDraw() {

		const ele = document.createElement('div');
		ele.setAttribute('class', "naviBase");

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "eleTarget",
			// "nextFcode" : "eleNext",

		};
		this.ele(eleDef);
		
		// To let this.items() have new this.ele().drawn as its target,
		// this.items should be refreshed .
		this.items(undefined);
		this.items().forEach(ele => ele.eleDraw());

	} // end of eleDraw

	// See function eleHandle()
	ele(...args) {

		if(this.eleObj == undefined){ this.eleObj = {}; }
		return eleHandle(this, this.eleObj, ...args);

	} // end of ele
	
} // end of class Navi

class NaviItem {
	// console.log("wc.js class NaviItem");

	constructor(page, eleTarget, navi, data) {
		this.page = page;
		this.eleTarget(eleTarget);
		this.navi = navi;
		this.dataObj = data;
	} // end of constructor

	// this.data()[0] : name, this.data()[1] : href
	data() {
		return this.dataObj;
	} // end of data
	
	eleTarget() {
		
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

		// let ele;
		// return this.eleTarget(ele);

	} // end of eleTarget

	// index number of this in this.navi.items() 
	index() {
		return this.navi.itemIndex(this);
	} // end of index

	// The next element where this.ele() is appended before .
	// If this.ele() is to be the last of this.eleTarget().children ,
	// return undefined of remove this.eleNext code .
	eleNext() {

		let itemNext = this.navi.itemNext(this);
		if(itemNext){ return itemNext.ele().drawn; }

	} // end of eleNext
	
	eleDraw() {
		// console.log("wc.js class NaviItem eleDraw");

		let ele = this.eleItem();
		let eleDef = {
			"ele" : ele,
			"targetFcode" : "eleTarget",
			"nextFcode" : "eleNext",
		};

		this.ele(eleDef);

	} // end of eleDraw

	eleItem() {

		let ele = document.createElement('span');
		ele.setAttribute('class', "naviItem");
		ele.setAttribute('data-json_id', this.index());

		let a = document.createElement('a');
		a.setAttribute('class', "naviAnchor");
		let href = this.data()[1];
		if(
			href != undefined
				&&
				0 < href.length
		){
			a.setAttribute('href', href);
		}

		let name = document.createTextNode(this.data()[0]);
		a.appendChild(name);

		ele.appendChild(a);

		// next navi's ele will be drawn after this .
		if(this.navi.itemNext(this)){
			ele.appendChild(navi_arrow());
		}

		return ele;
		
	} // end of eleItem
	
	// See function eleHandle()
	ele(...args) {

		if(this.eleObj == undefined){ this.eleObj = {}; }
		return eleHandle(this, this.eleObj, ...args);

	} // end of ele

	// See function eleHandle()
	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

	// This editor_optn is called by class Editor::editor_open
	editor_open() {
		// console.log("wc.js class NaviItem editor_open");

		 if(this.insertNew){
			 return this.editor_insert_open();
		 }
		// if(this.isBlank){
			// return this.editor_insert_open();
		// }

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorNaviItemTitle);
		html = html.replace("<!--placeHolder-->", htmlEditorEnter);
		html = html.replace("<!--placeHolder-->", htmlEditorMove);

		let ele = this.page.editor().ele_setup(this, html);
		ele = this.editorDataSet(ele)

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "editorTarget",
		};

		this.editorEle(eleDef);
		// this.ele().appendChild(this.editor_ele());

	} // end of editor_open

	editorDataSet(ele) {
		// console.log("wc.js class NaviItem editorDataSet");

		// Incase this.editor_insert_open(), this.data() is undefined
		if(this.index() == undefined){ return ele; }
		
		// let editor_ele = this.editor_ele();
		ele.querySelector(".inputTitle").value = this.data()[0];
		ele.querySelector(".inputHref").value = this.data()[1];

		return ele;
		
	} // end of editorDataSet

	editorTarget() {
		return this.ele().drawn;
	} // end of editorTarget


	// See function eleHandle()
	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

 	editor_enter() {

		// Insert new
		// a temporary NaviItem made at this.editor_insertMenu()
		if(this.insertNew){
			this.editor_insert_enter();
			return;
		}

		// ele of temporary NaviItem made at this.editor_insert_open() 
		// let editor_ele = this.editor_ele();
		let editor_ele = this.editorEle().drawn;

		let ele;
	    ele = editor_ele.querySelector(".inputTitle");
		this.data()[0] = ele.value;
	    ele = editor_ele.querySelector(".inputHref");
		this.data()[1] = ele.value;

		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
		this.eleDraw();

	} // end of editor_enter
	
	editor_moveMenu(event) {

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorMoveTo);

		editorMoveEventListenerSet(this, this.page.navi().items());

		// let editor_ele = this.page.editor().ele_setup(this, html);
		let ele = this.page.editor().ele_setup(this, html);

		// this.editor_ele(editor_ele)
		this.editorEle({"ele" : ele});
		
		event.stopPropagation(); // prevent editor_move

	} // end of editor_moveMenu
	
	editor_move(target, event) {
		// console.log("wc.js class NaviItem editor_move");
		
		let navi_data = [];
		let items = this.page.navi().items();
		for(let i = 0; i < items.length; i++){
			// skip self
			if(items[i] == this){ continue;}
			
			navi_data.push(items[i].data());
			// push data to move after target
			if(items[i] == target){
				navi_data.push(this.data());
			}
		}

		// this.eleEditor({"ele" : undefined});
		this.page.editor().editor_close(this);

		this.page.page_json["data"]["navi"] = navi_data;

		this.page.navi().items(undefined);

		this.page.navi().eleDraw();
		
 		this.page.editor().menu().changed(true);

		preventContextmenu(event); // prevent to go to the link

	} // end of editor_move
	
	editor_insertMenu() {
		
		// close the editor that requested this insert
		this.page.editor().editor_close(this);

		// Make a temporary NaviItem for this.editor_insert_open() .
		let naviItemTemp = new NaviItem(this.page, this.eleTarget(), this.navi);
		naviItemTemp.insertNew = true;
		naviItemTemp.insertTarget = this;

		this.page.editor().editor_open(naviItemTemp);

	} // end of editor_insertMenu

	// a temporary NaviItem made at this.editor_insertMenu()
	editor_insert_open() {
		// console.log("wc.js class NaviItem editor_insert_open ");

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorNaviItemTitle);
		html = html.replace("<!--placeHolder-->", htmlEditorEnter);

		let ele = this.page.editor().ele_setup(this, html);
		
		eleVisibleSet(
			ele,
			{
				'editor_delete' : 0
			}
		);

		// this.editor_ele(editor_ele);
		let eleDef = {
			"ele" : ele,
			"targetFcode" : "editorInsertTarget",
		};
		this.editorEle(eleDef);

		// this.insertTarget.ele().appendChild(editor_ele);
		
	} // end of editor_insert_open

	editorInsertTarget() {
		return this.insertTarget.ele().drawn;
	} // end of editorInsertTarget

	// a temporary NaviItem made at this.editor_insertMenu()
	editor_insert_enter() {
		// console.log("wc.js class NaviItem editor_insert_enter()");

		// this.editor_ele():
		//     a temporary NaviItem made at this.editor_insertMenu()
		let ele = this.editorEle().drawn;
	    let title = ele.querySelector(".inputTitle").value;
	    let href = ele.querySelector(".inputHref").value;

		let itemNew = this.navi.itemNew(this.insertTarget);
		itemNew.data()[0] = title;
		itemNew.data()[1] = href;

		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);

		this.navi.eleDraw();

	} // end of editor_insert_enter

	editor_delete() {
		//console.log("wc.js class NaviItem editor_delete");

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorNaviItemTitle);
		html = html.replace("<!--placeHolder-->", htmlEditorMove);
		
		let ele = this.page.editor().ele_setup(this, html);
		ele = this.editorDataSet(ele)

		eleVisibleSet(
			ele,
			{
				'editor_hr' : 0
				,'editor_delete' : 0
				,'editor_moveMenu' : 0
				,'editor_insertMenu' : 0
				,'editor_deleteConfirm' : 1
			}
		);

		this.editorEle({"ele": ele});
		
	} // end of editor_delete

	editor_deleteCancel() {

		this.page.editor().editor_close();
		
	} // end of editor_deleteCancel

	editor_deleteExecute() {
		
		let dataNavi2 = [];
		let dataNavi = this.page.page_json["data"]["navi"];
		for(let i = 0; i < dataNavi.length; i++){
			// if(i == this.navi_index){ continue;}
			if(i == this.index()){ continue;}
			dataNavi2.push(dataNavi[i]);
		}
		
		this.page.page_json["data"]["navi"] = dataNavi2;

		this.page.navi().items(undefined);

		this.page.editor().editor_close();
 		this.page.editor().menu().changed(true);

		this.page.navi().eleDraw();

	} // end of editor_deleteExecute

} // end of class NaviItem

const htmlEditorBox = `
<table  class="editTable">
<!--placeHolder-->
</table>
`; // end of const htmlEditorBox

const htmlEditorNaviItemTitle = `
	<tr>
	  <td>title</td>
	  <td><input class="inputTitle"></td>
	</tr>
	<tr>
	  <td>href</td>
	  <td>
	    <input class="inputHref">
	  </td>
	</tr>
<!--placeHolder-->
`; // end of const htmlEditorNaviItemTitle

const htmlEditorEnter = `
	<tr>
	  <td></td>
      <td>
	    <input type="button" class="editor_enter" value="Enter"> 
	    <input type="button" class="editor_cancel" value="Cancel">
	    <input type="button" value="New Page" class="editor_newPage invisible">
	  </td>
	</tr>
<!--placeHolder-->
`; // end of const htmlEditorEnter

const htmlEditorMove = `
	<tr>
	  <td></td>
	  <td><hr class="editor_hr"></td>
	</tr>

	<tr><td></td><td><iframe class="invisible iframe0"></iframe></td></tr>
      
	<tr>
	  <td></td>
	  <td>
        <input type="button" class="editor_moveMenu" value="Move"> 

        <input type="button" class="editor_insertMenu" value="Insert">

	    <input type="button" class="editor_delete" value="Delete">
	    <div class="editor_deleteConfirm invisible testColor">
	      <div class="editDeleteMessage"></div>
	      Delete , sure ?
	      <input type="button" class="editor_deleteExecute" value="Execute">
	      <input type="button" class="editor_deleteCancel" value="Cancel">
	    </div>

      </td>
	</tr>
`; // end of const htmlEditorMove

const htmlEditorMoveTo = `
<tr>
<td></td>
<td>Select where move to</td>
</tr>

<tr>
<td></td>
<td><input type="button" class="editor_cancel" value="Cancel"></td>
</tr>
<!--placeHolder-->
`; // end of const htmlEditorMoveTo

function navi_arrow(){
	// <span class="naviArrow">&nbsp;&gt;&nbsp;</span>

	let span = document.createElement('span');
	span.setAttribute('class', "naviArrow");
	span.innerHTML = "&nbsp;&gt;&nbsp;"

	return span

} // end of function navi_arrow

class Subsection {
	// console.log("wc.js class Subsection");

	constructor(page, id) {
		this.page = page;
		this.id = id;

		if(this.data()){ this.data().id = id; }
		
	} // end of constructor

	data() {
		return this.page.page_json["data"]["subsection"]["data"][this.id];
	} // end of data

	// this.data()["href"] starts with #
	hrefStartsSharp() {
		return this.data().href.match(/^#(.+)/);
	} // end of hrefStartsSharp

	contents() {
		// console.log("wc.js class Subsection contents()");

		if(0 < arguments.length){
			this.contentsObj = arguments[0];
			return this.contentsObj;
		}

		if(this.contentsObj){ return this.contentsObj; }

		// <div class="subsectionContent">
		let eleTarget = this.ele().drawn.querySelector(".subsectionContent");
		let contents = [];
		let page = this.page;
		let subsection = this;
		this.data()["content"].forEach(
			function(item, index, array){
				let content = new SubsectionContent(page, subsection, index);
				content.eleTarget(eleTarget);
				contents.push(content);
			}
		);

		return this.contents(contents);

	} // end of contents
	
	contentBlank() {
		// console.log("wc.js class Subsection contentBlank()");
		
		if(0 < arguments.length){
			this.contentBlankObj = arguments[0];
			return this.contentBlankObj;
		}

		if(this.contentBlankObj){ return this.contentBlankObj; }

		let index = undefined;
		let content = new SubsectionContent(this.page, this, index);
		content.data({"type" : "text", "value" : "no content"});
		content.eleTarget(this.ele().drawn.querySelector(".subsectionContent"));
		content.isBlank = true;

		return this.contentBlank(content);

	} // end of contentBlank

	// push the new content to 
	// this.page.page_json["data"]["subsection"]["data"]["content"];
	contentInsert(content) {
		// console.log("wc.js class Subsection contentInsert");

		if(! content.isBlank){ return; }

		// if content.insertTarget is defined  and found ,
		// put new content after the target ,
		// otherwise put after the last .
		let targetIndex;
		let contents = this.contents();
		if(content.insertTarget){
			for(let i =0; i< contents.length; i++){
				if(contents[i] == content.insertTarget){
					// put it after the target
					targetIndex = i + 1;
					break;
				}
			}
		}
		else {
			// put it at after the last
			targetIndex = contents.length;
		}

		// insert data
		let subsectionsData = this.page.page_json["data"]["subsection"]["data"];
		let contentsData = subsectionsData[this.id]["content"];
		contentsData.splice(targetIndex, 0, content.data());

		// Clear this.contents because index of SubsectionContent was changed 
		this.contents(undefined);

	} // end of contentInsert

	parent() {
		return this.page.subsection(this.data()["parent"]);
	} // end of parent

	children() {

		if(0 < arguments.length){
			this.childrenObj = arguments[0];
			return this.childrenObj;
		}

		if(this.childrenObj){ return this.childrenObj; }

		let children = [];
		let page = this.page;
		this.data()["child"].forEach(function(i){
			children.push(page.subsection(i));
		});

		return this.children(children);
		
	} // end of children

	childrenIndex() {

		if(0 < arguments.length){
			this.childrenIndexObj = arguments[0];
			return this.childrenIndexObj;
		}

		if(this.childrenIndexObj){ return this.childrenIndexObj; }

		let childrenIndex = new SubsectionIndex(this.page, this);
		return this.childrenIndex(childrenIndex);
		
	} // end of childrenIndex

	indexItem() {
		if(0 < arguments.length){
			this.indexItemObj = arguments[0];
			return this.indexItemObj;
		}

		if(this.indexItemObj){ return this.indexItemObj; }

		let indexItem = new SubsectionIndexItem(this.page, this);

		return this.indexItem(indexItem);

	} // end of indexItem

	childrenEleDraw() {
		this.children().forEach(child => child.eleDraw());
	} // end of childrenEleDraw

	eleTarget() {
		// console.log("wc.js class Subsection eleTarget()");
		
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

	} // end of eleTarget
	
	// The next element where this.ele() is appended before .
	// If just next subsection does not have valid ele().drawn,
	// find any ele().drawn valid following in the order .
	// If this.ele() is to be the last of this.eleTarget().children ,
	// return undefined of remove this.eleNext code .
	eleNext() {
		// console.log("wc.js class Subsection eleNext()");

		// eleNext is called every subsection.eleDraw()
		// Consider to escape repitation of the call .

		if(this.parent() == undefined){ return; }
		
		// Find out next subsection of this
		let thisFound;
		let subsectionNext;
		let children = this.parent().children();

		for(let i = 0; i < children.length; i++ ){

			// If thisFound is true
			// the loop is after this was found .
			if(thisFound){

				// Search only for href to local page, so ,
				// if href does not start with # go next loop
				if(children[i].data().href.slice(0, 1) != "#"){ continue; }
				
				// href starts with #
				
				// if(children[i].drawn == undefined){
				if(children[i].ele().drawn == undefined){

					// If children[i].drawn == undefined
					// its child should no be exists .
					// If it is ture, code followind is not necessary .
					const children2 = children[i].children();
					for(let j = 0; j < children2.length; j++){
						let eleNext = children2[j].eleNext()
						// if(eleNext && eleNext.ele().drawn){
						if(eleNext){
							subsectionNext = children2[j];
							break;
						}
					}

					if(subsectionNext){ break; }

					// let childNext = children[i].children().forEach(
						// subsection => subsection.eleNxt()
					// );

					continue;

				}
				
				// Case children[i].ele().drawn is defined some
				subsectionNext = children[i];
				break;
			}
			// find myself
			if(children[i] == this){
				thisFound = true; }
		}
		if(subsectionNext){ return subsectionNext.ele().drawn; }

		// if no ele().drawn valid, next would be some in this.parent() .
		return this.parent().eleNext();

	} // end of eleNext
	
	eleDraw() {
		// console.log("wc.js class Subsection eleDraw()");
		
		// not href = "#xxx", not local link
		if(this.hrefStartsSharp() == undefined){ return; }
		
		let ele = this.eleCreate();

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "eleTarget",
			"nextFcode" : "eleNext",
		};
		this.ele(eleDef);
		
		// subsection contents
		// clear this.contents() to rerlesh SubsectionContent.eleTarget()
		// this.contents(undefined);

		this.contents().forEach(content => content.eleDraw());

		// If no content, put contentBlank
		// to give a switch to open editor
		if(this.contents().length == 0){
			// clear contentBlank
			// That is need to reset this.contentBlank.eleTarget
			// that's element  might be deleted and need to set new one .
			this.contentBlank(undefined);
			this.contentBlank().eleDraw();
		}

		// this.children().forEach(child => child.eleDraw());
		this.childrenEleDraw();

	} // end of eleDraw

	eleCreate() {

		let data = this.data();

		// <div class="subsection" data-json_id="0">
		let ele = document.createElement('div');
		ele.setAttribute('class', "subsection");
		ele.setAttribute('data-json_id', this.id);

		// data["href"] that does not start with # is rejected in this.eleDraw() .
		// idHref is based href, not subsection id .
		let idHref = data["href"].replace(/^#/, '');
		ele.setAttribute('id', idHref);

		// <a href="javascript:history.back();">back</a>
		let backA = document.createElement('a');
		backA.setAttribute('class', "subsection");
		backA.setAttribute('href', "javascript:history.back()");
		backA.appendChild(document.createTextNode("back"));
		ele.appendChild(backA);
		
		// space
		ele.appendChild(document.createTextNode(" "));

		// <a href="#top">Top</a>
		let topA = document.createElement('a');
		// topA.setAttribute('href', "#top");
		topA.setAttribute('href', "#");
		topA.appendChild(document.createTextNode("Top"));
		ele.appendChild(topA);

		// <div class="subsectionTitle">Title</div>
		let titleDiv = document.createElement('div');
		titleDiv.setAttribute('class', "subsectionTitle");

		titleDiv.appendChild(document.createTextNode(data["title"]));

		ele.appendChild(titleDiv);

		// <div class="subsectionContent">
		let contentDiv = document.createElement('div');
		contentDiv.setAttribute('class', "subsectionContent");
		ele.appendChild(contentDiv);

		return ele;
		
	} // end of eleCreate
	
	// See function eleHandle()
	ele(...args) {

		if(this.eleObj == undefined){ this.eleObj = {}; }
		return eleHandle(this, this.eleObj, ...args);

	} // end of ele

	// This editor_optn is called by class Editor::editor_open
	editor_open() {

		let html = htmlEditorBox;
		// html = html.replace("<!--placeHolder-->", htmlEditorTitle);
		html = html.replace("<!--placeHolder-->", htmlEditorSubsectionContent);
		html = html.replace("<!--placeHolder-->", htmlEditorEnter);
		let ele = this.page.editor().ele_setup(this, html);
		
		eleVisibleSet(
			ele,
			{
				'editor_enter' : 1
				, 'editor_cancel' : 1
				,'editor_delete' : 1
				,'editor_deleteConfirm' : 0
				,'editor_moveMenu' : 0
				, 'editor_subsection_content_type' : 0
			}
		);

		this.editorDataSet(ele);

		let eleDef = {
			"ele" : ele,
			"nextFcode" : "editorNext", // nextFcode is prier than targetFcode
			"targetFcode" : "editorTarget",
		};

		this.editorEle(eleDef);

	} // end of editor_open

	editorNext() {
		
		let eleContent = this.ele().drawn.querySelector(".subsectionContent");
		return eleContent.childNodes[0];
		
	} // end of editorNext

	editorTarget() {

		return this.ele().drawn.querySelector(".subsectionContent");
		
	} // end of editorTarget
	
	// See function eleHandle()
	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

	editorDataSet(ele) {

		// let ele = this.editor_ele();

		// title
		ele.querySelector(".title").innerHTML = this.data()["title"];

		let eleContent = ele.querySelector(".editor_subsection_content");
		eleContent.textContent = JSON.stringify(this.data()["content"]);
		
	} // end of editorDataSet

 	editor_enter() {

		let editor_ele = this.editorEle().drawn;
	    let contentEle = editor_ele.querySelector(".editor_subsection_content");
		
		const f0 = new Function('return '+ contentEle.value +';');

		this.data()["content"] = f0();
		this.contents(undefined);

		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
		// eleRefresh(this);
		this.eleDraw();

	} // end of editor_enter

	childNew() {
		let child = this.page.subsectionNew();
		child.data().parent = this.id;





		
		return child;
	} // end of childNew

	// create a new subsection and push it into child
	// and return the subsection
	child_insert(target_subsection, insert_direction) {
		// console.log("wc.js class Subsection child_insert()");
		
		// insert_direction: "before" / "after"

		let subsectionNew = this.childNew();
		let idNew = subsectionNew.id;
		let child = this.data().child;
		// let target_id = target_subsection.id;
		let target_id = target_subsection ? target_subsection.id : undefined;
		let idPos;
		for(let i = 0; i < child.length; i++){
			if(child[i] == target_id){
				if(insert_direction == "before"){
					idPos = i;
				}else{
					// "after"
					idPos = i + 1;
				}
				break;
			}
		}

		if(idPos == undefined){ child.push(idNew); }
		else{ child.splice(idPos, 0, idNew); }

		// Clear the previous children data
		this.children(undefined);

		this.childrenIndex().list(undefined);

		return subsectionNew;
		
	} // end of child_insert

	oneChildAtleast() {
		// console.log("wc.js class Subsection oneChildAtleast()");
		
		this.children(undefined);
		// Atleaset one subsection exists
		if(this.children().length != 0){ return;}

		let childNew = this.child_insert();
		childNew.data().title = "subsection";
		childNew.data().href = "#temp";
		// contentBlank will be used

		childNew.indexItem().eleDraw();
		childNew.eleDraw();
		
	} // end of oneChildAtleast

	clear() {
		// console.log("wc.js class Subsection clear()");

		// console.log("wc.js class Subsection clear() id:" + this.id);
		
		// Remove this.ele().drawn
		this.ele({"ele" : undefined});

		// Remove indexItem
		this.indexItem().ele({"ele" : undefined});

		// Remove children
		this.children().forEach(child => child.clear());

		// Delete page.subsection(this.id)
		this.page.subsection(this.id, undefined);
		
	} // end of clear
	
} // end of class Subsection

const htmlEditorSubsectionContent = `
<tr>
	  <td>title:</td>
	  <td><span class="title"></span></td>
	</tr>

<tr class="editor_subsection_content_type">
	  <td></td>
	  <td>
	    <select name="editor_subsection_content_type">
	      <option value="html">HTML</option>
	      <option value="text">Text</option>
	      <option value="script">Script</option>
	    </select>
	  </td>
	</tr>

	<tr>
	  <td></td>
	  <td><textarea class="editor_subsection_content"></textarea></td>
	</tr>
<!--placeHolder-->
`; // end of const htmlEditorSubsectionContent

class SubsectionIndex {
	// console.log("wc.js class SubsectionIndex");

	constructor(page, subsection) {
		this.page = page;
		this.subsection = subsection;
	} // end of constructor

	// List of subsections
	list() {
		
		if(0 < arguments.length){
			this.listObj = arguments[0];
			return this.listObj;
		}

		if(this.listObj){ return this.listObj; }

		return this.list(this.subsection.children());

	} // end of list
	
	// In case of children of this.page.subsectionTop() ,
	// an eleTarget is given as an argument.
	// Otherwise eleTarget is the element li of parent subsection .
	eleTarget() {
		
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

		// Target is li element of parent subsection .
		let ele = this.list()[0].parent().indexItem().ele().drawn;
		return this.eleTarget(ele);

	} // end of eleTarget

	eleDraw() {
		// console.log("wc.js class SubsectionIndex eleDraw()");

		// console.log("wc.js class SubsectionIndex eleDraw() id:" + this.subsection.id);

		if(this.subsection.children().length == 0){

			// If no subsection, add one subsection
			// to let show one subsection that make you possible to edit operation
			if(this.subsection.id == 0){
				
				const ele = document.createElement('ul');
				ele.setAttribute('class', "listItemBase");
				const eleDef = {
					"ele" : ele,
					"targetFcode" : "eleTarget"
				};
				
				this.ele(eleDef);

				// return;

				// console.log("wc.js class SubsectionIndex eleDraw() this.subsection.id:" + this.subsection.id);

				// let childNew = this.subsection.child_insert();

				// childNew.data("href", "HF");
				// childNew.data().title = "subsection";
				// childNew.data().href = "#_";
				// childNew.data().content = [{"type":"text", "value": "content"}];
				

			}

			// else {
				// return;
			// }

			
			return;
		}
		
		const ele = document.createElement('ul');
		ele.setAttribute('class', "listItemBase");
		const eleDef = {
			"ele" : ele,
			"targetFcode" : "eleTarget"
		};

		this.ele(eleDef);

		console.log("wc.js class SubsectionIndex eleDraw() list.length:" + this.list().length);
		

		this.list().forEach(subsection => subsection.indexItem().eleDraw());

	} // end of eleDraw
	
	// See function eleHandle()
	ele(...args) {

		if(this.eleObj == undefined){ this.eleObj = {}; }
		let obj = this.eleObj;

		return eleHandle(this, obj, ...args);

	} // end of ele

	// This editor_optn is called by class Editor::editor_open
	editor_open() {
		// console.log("wc.js class SubsectionIndex editor_open()");

		// case new subsection to be inserted
		if(this.subsection.insert_direction){
			return this.editor_insert_open();
		}
		
		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorTitleInput);
		html = html.replace("<!--placeHolder-->", htmlEditorEnter);
		html = html.replace("<!--placeHolder-->", htmlEditorMove);
		let ele = this.page.editor().ele_setup(this, html);
		this.editorDataSet(ele);

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "editorTarget",
		};
		this.editorEle(eleDef);
		
	} // end of editor_open

	editorTarget() {
		return this.ele().drawn;
	} // end of editorTarget

	// See function eleHandle()
	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

	editorDataSet(ele) {

		let data = this.subsection.data();

		// In case not insert
		if(data){
			editorDataSet(ele,
						  {
							  "inputTitle" : data["title"],
							  "inputHref" : data["href"],
						  }
						 );
		}
		
	} // end of editorDataSet

} // end of class SubsectionIndex

const htmlEditorTitle = `
	<tr>
	  <td>title:</td>
	  <td><span class="editorTitle"></span></td>
	</tr>
<!--placeHolder-->
`; // end of const htmlEditorTitle

const htmlEditorTitleInput = `
	<tr>
	  <td>title</td>
	  <td><input class="inputTitle"></td>
	</tr>
	<tr>
	  <td>href</td>
	  <td>
	    <input class="inputHref">
	  </td>
	</tr>

	<tr>
	  <td></td>
	  <td>
	    <input type="button" value="Sub List" class="invisible editSublistCreate">
	  </td>
	</tr>
<!--placeHolder-->
`; // end of const htmlEditorTitleInput

class SubsectionIndexItem {
	// console.log("wc.js class SubsectionIndexItem");

	constructor(page, subsection) {
		this.page = page;
		this.subsection = subsection;

	} // end of constructor

	eleTarget() {
		// console.log("wc.js class SubsectionIndexItem eleTarget()");
		
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){
			return this.eleTargetObj;
		}
		
		// console.log("wc.js class SubsectionIndexItem eleTarget() subsectin.id: " + this.subsection.id);
		
		let ele = this.subsection.parent().childrenIndex().ele().drawn;

		return this.eleTarget(ele);

	} // end of eleTarget

	// The next element where this.ele() is appended before .
	// If this.ele() is to be the last of this.eleTarget().children ,
	// return undefined of remove this.eleNext code .
	eleNext() {
		// console.log("wc.js class SubsectionIndexItem eleNext()");

		// console.log("wc.js class SubsectionIndexItem eleNext() this.subsection.id: " + this.subsection.id);

		let match = false;
		for(const id of this.subsection.parent().data().child){
			// This happens on next id of this.subsection.id
			if(match){
				return this.page.subsection(id).indexItem().ele().drawn;
			}
			// If next loop happens, it proceeds with math: true;
			if(this.subsection.id == id){ match = true;}		
		}

		return false;
		
	} // end of eleNext
	
	eleDraw() {
		// console.log("wc.js class SubsectionIndexItem eleDraw()");
		
		// console.log("wc.js class SubsectionIndexItem eleDraw() subsectin.id:" + this.subsection.id);
		
		let ele = document.createElement('li');
		ele.setAttribute('class', "subsectionListItem");
		ele.setAttribute('data-json_id', this.subsection.id);

		let data = this.subsection.data();
		let a = document.createElement('a');
		a.setAttribute('class', "title");
		a.setAttribute('href', data["href"]);

		a.appendChild(document.createTextNode(data["title"]));
		ele.appendChild(a);

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "eleTarget",
			"nextFcode" : "eleNext",
		};

		this.ele(eleDef);

		// console.log("wc.js class SubsectionIndexItem eleDraw() calling childrenIndex().eleDraw() subsection.id:" + this.subsection.id);
		
		this.subsection.childrenIndex().eleDraw();
		
	} // end of eleDraw
	
	// See function eleHandle()
	ele(...args) {

		if(this.eleObj == undefined){ this.eleObj = {}; }
		let obj = this.eleObj;

		return eleHandle(this, obj, ...args);

	} // end of ele

	// See function eleHandle()
	editorEle(...args) {
		// console.log("wc.js class SubsectionIndexItem editorEle()");

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

	// This is called by class Editor::editor_open
	editor_open() {

		// a temporary Subsection made at this.editor_insertMenu_direction()
		if(this.subsection.insert_direction){
			return this.editor_insert_open();
		}

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorTitleInput);
		html = html.replace("<!--placeHolder-->", htmlEditorEnter);
		html = html.replace("<!--placeHolder-->", htmlEditorMove);
		let ele = this.page.editor().ele_setup(this, html);

		eleVisibleSet(ele, {'editor_newPage' : 1});
		
		this.editorDataSet(ele);

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "editorTarget",
		};

		this.editorEle(eleDef);

	} // end of editor_open

	editorTarget() {
		return this.ele().drawn;
	} // end of editorTarget
	
	editorDataSet(ele) {

		let data = this.subsection.data();

		let toSet = {};
		for(const name of ["title", "href"]){
			let v = data[name];
			if(v == undefined){ v = ""; }
			// key: "name" to "inputName"
			toSet["input"+name[0].toUpperCase()+name.slice(1)] = v;
		}

		editorDataSet(ele, toSet);
		
	} // end of editorDataSet
	
	editor_enter() {
		// console.log("wc.js class SubsectionIndexItem editor_enter()");

		// a temporary Subsection made at this.editor_insertMenu_direction()
		if(this.subsection.insert_direction){
			this.editor_insert_enter();
			return;
		}

		let ele = this.editorEle().drawn.querySelector(".inputTitle");
		this.subsection.data()["title"] = ele.value;

		this.editor_enter_href();
		
		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
		// apply the change to SubsectionIndexItem display
		this.eleDraw();

		// apply change to subsection diplay
		// href: ^#...
		if(this.subsection.hrefStartsSharp()){
			this.subsection.eleDraw();
		}

	} // end of editor_enter

	editor_enter_href() {
		// console.log("wc.js class SubsectionIndexItem editor_enter_href()");

		// href
		// 
		// undef to #subtitle0
		// undef to ./abc.html
		//   this.data()["href"] is expected to have some value,
		//   otherwise must be handled by this.editor_insert_enter();
		
		// #subtitle0 to #subtitle0 // no change
		// #subtitle0 to #subtitle1
		// #subtitle0 to abc.html // not allow, must use delete
		// #subtitle0 to undefined // not allow, must use delete
		// abc.html to abc.html // no change
		// abc.html to xyz.html
		// abc.html to #subtitle0 // not allow, mut use delete
		// abc.html to undefined // not allow, mut use delete

		// let editor_ele = this.editor_ele();
	    // let ele = editor_ele.querySelector(".inputHref");
		let ele = this.editorEle().drawn.querySelector(".inputHref");
		const href = ele.value;

		if(href == "" || href == "#"){ return;}

		let page_json = this.page.page_json;

		// #subtitle0 to 
		if(this.subsection.hrefStartsSharp()){
			// this.subsection.data()["href"] : #abc
			
			// #subtitle0 to #subtitle0 // no change
			if(this.subsection.data()["href"] == href){ return;}

			// #subtitle0 to #subtitle1
			if(href.match(/^#/)){
				if(href_in_use(page_json, href)){ return; }
				this.subsection.data()["href"] = href;
				return;
			}			
			// #subtitle0 to abc.html // not allow, must use delete
			return;
		} else {
			// abc.html to

			// abc.html to abc.html // no change
			if(this.subsection.data()["href"] == href){ return;}

			// abc.html to #subtitle0 // not allow, mut use delete
			if(href.match(/^#/)){ return; }

			// abc.html to xyz.html
			this.subsection.data()["href"] = href;
		}
		
		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
	} // end of editor_enter_href

	editor_insertMenu() {
		// console.log("wc.js class SubsectionIndexItem editor_insertMenu()");

		this.editor_insertMenu_direction("after");

	} // end of editor_insertMenu

	// a temporary Subsection
	editor_insertMenu_direction(insert_direction) {
		// insert_direction: "before" / "after"

		// close the editor menu that requested this insert
		this.page.editor().editor_close(this);

		// a temporary Subsection
		// create a temporary Subsection that not exists yet to be inserted
		// to call a subsection, should use this.page().subsection(id)
		// but since this subsection not exists yet, use new Subsection
		let subsectionTemp = new Subsection(this.page, undefined);
		// memory where to be inserted to, and what on
		subsectionTemp.insertTarget = this.subsection;
		subsectionTemp.insert_direction = insert_direction;
		let indexItem = subsectionTemp.indexItem();
		// open new editor for new subsection indexItem
		this.page.editor().editor_open(indexItem);

	} // end of editor_insertMenu_direction

	// a temporary Subsection made at this.editor_insertMenu_direction()
	// Called by this.editor_open()
	editor_insert_open() {
		// console.log("wc.js class SubsectionIndexItem editor_insert_open()");

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->",
							htmlEditorSubmissionInsertTitle);
		let ele = this.page.editor().ele_setup(this, html);

		let eleDef = {"ele" : ele};
		eleDef["targetFcode"] = "editor_insertOpenTarget";

		// if(this.subsection.insert_direction == "before"){
			// eleDef["nextFcode"] = "editor_insertOpenTarget";
		// }

		this.editorEle(eleDef);
		
	} // end of editor_insert_open

	editor_insertOpenTarget() {
		return this.subsection.insertTarget.indexItem().ele().drawn;
	} // end of editor_insertOpenTarget

	// a temporary Subsection made at this.editor_insertMenu_direction()
	// subsectionTemp -> data -> subsectionNew
	editor_insert_enter() {
		// console.log("wc.js class SubsectionIndexItem editor_insert_enter");

		// this.subsection.insertTarget = undefined;
		// this.subsection.insert_direction = undefined;
		let target_subsection = this.subsection.insertTarget;
		let insert_direction = this.subsection.insert_direction;
		let parentSubsection = target_subsection.parent();
		let childSubsection = parentSubsection.child_insert(target_subsection,
														  insert_direction);

		// a temporary Subsection made at this.editor_insertMenu_direction()
	    let editor_ele = this.editorEle().drawn;
		// let ele;

		let href = editor_ele.querySelector(".inputHref").value;

		// Rather than reject the request if href is empty,
		// make a temporary value and create a subsection .
		// That is simpler and easier to be recognized
		// that there is no data by user .
		// Then user can enter some value or delete subsection with emply data .
		// 
		// no href value
		if(href.length == 0 || href == "#"){
			href =  hrefNotUsed(this.page);
		}

		let title = editor_ele.querySelector(".inputTitle").value;

		// case no title given
		if(title.length == 0){ title = href; }

		childSubsection.data()["title"] = title;
		childSubsection.data()["href"] = href;

		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
		childSubsection.eleDraw();
		childSubsection.indexItem().eleDraw();
		
		// const indexTarget = parentSubsection.indexItem.ele().drawn;
		// parentSubsection.childrenIndex().eleTarget(indexTarget);
		// parentSubsection.childrenIndex().eleDraw();
		
		// const indexItemTarget = parentSubsection.childrenIndex().ele().drawn;
		// childSubsection.indexItem().eleTarget(indexItemTarget);
		// childSubsection.indexItem().eleDraw();
		
	} // end of editor_insert_enter

	editor_moveMenu() {
		// console.log("wc.js class SubsectionIndexItem editor_moveMenu");

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorMoveTo);
		let ele = this.page.editor().ele_setup(this, html);

		this.editorMoveMenuEventListenerSet(this.page.subsectionTop(),
											this.subsection.id);

		this.editorEle({"ele" : ele});

		event.stopPropagation(); // prevent editor_move

	} // end of editor_moveMenu

	editorEleMoveTo() {
		// console.log("wc.js class SubsectionIndexItem editorEleMoveTo");

		let page2 = new Page(undefined, page_json);
		// let eleUl =  page2.subsectionTop().ele_children_ul()
		let eleUl =  page2.subsectionTop().children2().ele().drawn;

		let id_move = this.subsection.id;
		let aList = eleUl.getElementsByTagName("a");
		for(let eleA of aList){
			
			const f0_ref = new Function('return this.editor_move;');
			const f0 = f0_ref.apply(this);
			let caller = this;
			let id_to = eleA.getAttribute("data-json_id");
			eleA.addEventListener('click', function(event){
				f0.apply(caller, [id_move, id_to, event]);
			} );

		}
		
		return eleUl;

	} // end of editorEleMoveTo

	editorMoveMenuEventListenerSet(subsection, moveId) {

		if(subsection.id != 0){
			// ele li
			let indexItem = subsection.indexItem();
			let eleA = indexItem.ele().drawn.getElementsByTagName("a")[0];

			const f0_ref = new Function('return this.editor_move;');
			const f0 = f0_ref.apply(this);
			let caller = this;
			let toId =  subsection.id;
			
			// because function for the event has no name,
			// put the function into indexItem.editorMoveOn.action
			// to be able to remove the eventListener
			// at editorMoveMenuEventListenerRemove
			// listItem.editorMoveOn = {}
			indexItem.editorMoveOn = {
				trigger: "click",
				action: function (event){
					f0.apply(caller, [moveId, toId, event]);
				},
			}

			indexItem.ele().drawn.addEventListener(
				indexItem.editorMoveOn.trigger,							
				indexItem.editorMoveOn.action,
			);

		}

		for(let id of subsection.data()["child"]){
			this.editorMoveMenuEventListenerSet(this.page.subsection(id), moveId);
		}

	} // end of editorMoveMenuEventListenerSet
	
	editorMoveMenuEventListenerRemove(subsection) {
		// console.log("wc.js class SubsectionIndexItem editorMoveMenuEventListenerRemove()");
		if(subsection.id != 0){
			let indexItem = subsection.indexItem();
			indexItem.ele().drawn.removeEventListener(
				indexItem.editorMoveOn.trigger,							
				indexItem.editorMoveOn.action,
			);
		}

		for(let id of subsection.data()["child"]){
			this.editorMoveMenuEventListenerRemove(this.page.subsection(id));
		}

	} // end of editorMoveMenuEventListenerRemove

	editor_move(moveId, toId, event) {
		// console.log("wc.js class SubsectionIndexItem editor_move()");

		preventContextmenu(event); // prevent move to href;
		// Remove eventListener about move .
		this.editorMoveMenuEventListenerRemove(this.page.subsectionTop());
		this.page.editor().editor_close();

		// ignore Move to self .
		if(moveId == toId){ return;}

		let moveSubsection = this.page.subsection(moveId);
		let moveParent  = moveSubsection.parent();
		// let moveChildrenIds = moveParent.data().child;

		for(let i =0; i<moveParent.data().child.length; i++ ){
			if(moveParent.data().child[i] == moveId){
				moveParent.data().child.splice(i, 1);			
				break;
			}
		}

		moveParent.children(undefined);

		let toSubsection = this.page.subsection(toId);
		let toParent  = toSubsection.parent();

		// Set new parent id
		this.subsection.data().parent = toParent.id;
		
		// toParent.data().child
		for(let i=0; i<toParent.data().child.length; i++){
			if(toParent.data().child[i] == toId){
				toParent.data().child.splice(i+1, 0, moveId);
				break;
			}
		}

		toParent.children(undefined);

		moveSubsection.clear();			

		const moveSubsection2 = this.page.subsection(moveId);
		moveSubsection2.indexItem().eleDraw();
		moveSubsection2.eleDraw();

 		this.page.editor().menu().changed(true);

	} // end of editor_move

	editor_newPage() {
		// console.log("wc.js class SubsectionIndexItem editor_newPage()");		

		let data = {};

		let editor_ele = this.editorEle().drawn;
		
		let eleHref = editor_ele.querySelector(".inputHref");
		data["href"] = eleHref.value.trim();
		if(data["href"].length == 0){ return; }

		// start with #
		if(data["href"].slice(0, 1) == "#"){ return; }

	    let eleTitle = editor_ele.querySelector(".inputTitle");
		data["title"] = eleTitle.value.trim();
		
		let res = posData("page_new", data);
		res.then(data => {
			console.log("wc.jp function sditor_newPage res:" + data.res);
		});

		this.page.editor().editor_enter(this);
		
	} // end of editor_newPage

	editor_delete() {

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorTitle);
		html = html.replace("<!--placeHolder-->", htmlEditorMove);

		let ele = this.page.editor().ele_setup(this, html);

		editorDataSet(ele,
					  {"editorTitle" : this.subsection.data()["title"]});

		eleVisibleSet(
			ele,
			{
				'editor_insertMenu' : 0
				,'editor_moveMenu' : 0
				,'editor_delete' : 0
				,'editor_deleteConfirm' : 1
			}
		);

		this.editorEle({"ele" : ele});

	} // end of editor_delete
	
	editor_deleteCancel() {

		this.page.editor().editor_close();
		
	} // end of editor_deleteCancel

	editor_deleteExecute() {
		// console.log("wc.js class SubsectionIndexItem editor_deleteExecute()");

		this.page.editor().editor_close();

		let thisId = this.subsection.id;
		// let parent = this.page.subsection(this.subsection.data()["parent"]);
		let parent = this.subsection.parent();
		let child = [];
		let index;

		// scan to find index no of thisId;
		for(let i = 0; i<parent.data()["child"].length; i++){
			if(parent.data()["child"][i] == thisId){
				index = i;
				break;
			}
		}

		if(index == undefined){ return; }

		this.page.editor().editor_close();

		// Remove this SubsectionIndexItem
		this.ele({"ele" : undefined});
		// Remove subsection
		this.page.subsection(thisId).ele({"ele" : undefined});

		// delete thisId in the index of the data
		parent.data()["child"].splice(index, 1);

		// delete subsection data of thisId
		let data_base = this.page.page_json["data"]["subsection"]["data"];
		delete data_base[thisId];

		this.page.id_return(thisId);

 		this.page.editor().menu().changed(true);
		
		// only on subsectionTop
		// if no subsection, 
		this.page.subsectionTop().oneChildAtleast();
	
	} // end of editor_deleteExecute

	editor_subContent() {
		// console.log("wc.js class SubsectionIndexItem editor_subContent()");
		
		// this.insertTarget : parent subsection
		// let parent = this.insertTarget;
		let parentSubsection = this.subsection.insertTarget;
		let childSubsection = parentSubsection.child_insert();

		// a temporary Subsection made at this.editor_insertMenu_direction()
	    let editor_ele = this.editorEle().drawn;

		let href = editor_ele.querySelector(".inputHref").value;

		// Rather than reject the request if href is empty,
		// make a temporary value and create a subsection .
		// That is simpler and easier to be recognized
		// that there is no data by user .
		// Then user can enter some value or delete subsection with emply data .
		// 
		// no href value
		if(href.length == 0 || href == "#"){
			href =  hrefNotUsed(this.page);
		}

		let title = editor_ele.querySelector(".inputTitle").value;

		// case no title given
		if(title.length == 0){ title = href; }

		childSubsection.data()["title"] = title;
		childSubsection.data()["href"] = href;

		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
		childSubsection.eleDraw();

		const indexTarget = parentSubsection.indexItem().ele().drawn;
		parentSubsection.childrenIndex().eleTarget(indexTarget);
		
		// console.log("wc.js class SubsectionIndexItem editor_subContent() calling childrenIndex().eleDraw()");
		
		parentSubsection.childrenIndex().eleDraw();
		
		const indexItemTarget = parentSubsection.childrenIndex().ele().drawn;
		childSubsection.indexItem().eleTarget(indexItemTarget);
		childSubsection.indexItem().eleDraw();
		
	} // end of editor_subContent
		
} // end of class SubsectionIndexItem

const htmlEditorSubmissionInsertTitle = `
	<tr>
	  <td colspan=2>New Subsection</td>
	</tr>
	<tr>
	  <td>title</td>
	  <td><input class="inputTitle"></td>
	</tr>
	<tr>
	  <td>href</td>
	  <td>
	    <input class="inputHref" value="#">
	  </td>
	</tr>

	<tr>
	  <td></td>
	  <td>
	    <input type="button" value="New Page" class="editor_newPage">
	    <input type="button" value="Sub Content" class="editor_subContent">
	  </td>
	</tr>

	<tr>
	  <td></td><td>
	    <input type="button" class="editor_enter" value="Enter"> 
	    <input type="button" class="editor_cancel" value="Cancel">
	  </td>
	</tr>
`; // end of const htmlEditorSubmissionInsertTitle

function href_in_use(page_json, href) {

	for(let id in page_json["data"]["subsection"]["data"]){
		let subsection_json = page_json["data"]["subsection"]["data"][id];
		if(page_json["data"]["subsection"]["data"][id]["href"] == href){
			return true;
		}
	}

	return false;
	
} // end of function href_in_use

// Find href not used in this page
function hrefNotUsed(page) {
	// console.log("wc.js function hrefNotUsed");

	for(let i=1; i<100; i++){
		let hrefMatch = false;
		let href = "#undefined" + i;
		for(const key in page.page_json["data"]["subsection"]["data"]){
			const data = page.page_json["data"]["subsection"]["data"][key];
			if(data.href == href){
				hrefMatch = true;
				break;
			}
		}
		if(hrefMatch){ continue;}
		// no mached: not used .
		return href;
	}
		
} // end of function hrefNotUsed

class SubsectionContent {
	// console.log("wc.js class SubsectionContent");

	constructor(page, subsection, index) {
		this.page = page;
		this.subsection = subsection;
		this.index = index;
	} // end of constructor

	data() {

		if(0 < arguments.length){
			this.dataObj = arguments[0];
			return this.dataObj;
		}

		if(this.dataObj){ return this.dataObj; }

		let data = this.subsection.data()["content"][this.index];

		// for this.isBlank == true to keep values temporary .
		if(data == undefined){ data = {};}
		
		return this.data(data);
		
	} // end of data
	
	eleTarget() {
		
		if(0 < arguments.length){
			this.eleTargetObj = arguments[0];
			return this.eleTargetObj;
		}

		if(this.eleTargetObj){ return this.eleTargetObj; }

	} // end of eleTarget

	eleNext() {
		// console.log("wc.js class SubsectionContent eleNext()");
		// Get next content .
		let subsectionContentNext = this.subsection.contents()[this.index+1];
		if(subsectionContentNext == undefined){ return; }
		return subsectionContentNext.ele().drawn;
	} // end of eleNext

	eleDraw() {
		// console.log("wc.js class SubsectionContent eleDraw()");
		
		let ele;
		if(this.data()["type"] == "html"){
			ele = this.ele_html();
		}
		if(this.data()["type"] == "script"){
			ele = this.ele_script();
		}
		if(this.data()["type"] == "text"){
			ele = this.ele_text();
		}
		
		ele.setAttribute('data-json_id', this.subsection.id);

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "eleTarget",
			"nextFcode" : "eleNext",
		};

		this.ele(eleDef);

	} // end of eleDraw
	
	// See function eleHandle()
	ele(...args) {

		if(this.eleObj == undefined){ this.eleObj = {}; }
		return eleHandle(this, this.eleObj, ...args);

	} // end of ele

	ele_html() {
		// <div class="html">
		let div_html = document.createElement('div');
		div_html.setAttribute('class', "html subsectionContent");
		// div_html.innerHTML = this.data_to_html_content(this.data()["value"]);
		div_html.innerHTML = this.data()["value"];

		return div_html;
	} // end of ele_html

	ele_script() {
		// console.log("wc.js class SubsectionContent ele_script()");
		// <div class="text script">
		
		// to append this.editor_insert_open() not inside of class "script",
		// make clsss "subsectionContent" out of class "script"
		let div_content = document.createElement('div');
		div_content.setAttribute('class', "subsectionContent");

		let div_script = document.createElement('div');
		div_script.setAttribute('class', "script");

		// let data = this.data_to_html(this.data()["value"]);
		// div_script.innerHTML = this.data_to_text_content(data);
		div_script.innerHTML = this.data_to_text_content();
		
		div_content.appendChild(div_script);
		
		return div_content;
	} // end of ele_script

	ele_text() {
		// <div class="text">
		let div_text = document.createElement('div');
		div_text.setAttribute('class', "text subsectionContent");
		// div_text.innerHTML = this.data_to_html(this.data()["value"]);

		// div_text.innerHTML = this.data()["value"];
		div_text.innerHTML = this.data_to_text_content();
		
		return div_text;
	} // end of ele_text

	// Convert \< or \> to &lt;, &gt;
	// Convert \n \n\n to <br> <p></p>
	data_to_text_content() {

		let data = this.data()["value"];

		// Convert \< or \> to &lt;, &gt;
		let str = textAngleToEntity(data);

		// <> are handled as html element.
		// But any space is handled as text,
		// espacially \n will be converted to <br>
		// Considering html, \n between element eg; <>\n<>
		// should not be handle as <br>, but ignored.
		// Convert >\n< to >< removing \n, spaces around \n as well .
		// But >\n\n< will not be ignored .
		// It is considered as intended to put returns between the elements .
		// eg: <hr>\n\n<hr>
		// str = str.replace(/>\s+</, "><");
		str = str.replaceAll(/>[ ]*\n[ ]*</g, "><");
		// Since two \n required to set <br>,
		// remove one \n so that one return can be set.
		// otherwise two \n is minimum returns .
		str = str.replaceAll(/>\n/g, ">");

		// // Convert ##..$ to xxx, in <... href="##..$">
		// str = this.page.href_reference().href_set(str);

		// Convert \n \n\n to <br> <p></p>
		str = text_to_html2(str);

		return str;

	} // end of data_to_text_content

	// Find tag part <...> from data
	// and replace with resut of
	// this.page.href_reference().tag_set(parts_gt[0], this.page)
	// parts_gt[0] : tag part <...>
	data_to_html(data) {
		// console.log("wc.js class SubsectionContent data_to_html ");

		let data_str = data;
		let html = "";

		while(0 < data_str.length) {
			// text part + <...
			// divide by "<"
			let parts_lt = divide_by_lt(data_str);
			// parts_lt[0] : text part
			// parts_lt[1] : <...
			
			// text part
			// html += text_to_html2(parts_lt[0]);
			html += parts_lt[0];

			// "<" and the rest
			// element part
			// no element part
			// if (parts_lt[1].length < 0) { break; }
			if (parts_lt[1].length == 0) { break; }

			// <...> + ...
			// devide by ">"
			let parts_gt = divide_by_gt(parts_lt[1]);
			// parts_gt[0] : <...>
			// parts_gt[1] : ...
			
			// element tag
			// html += parts_gt[0];
			html += this.page.href_reference().tag_set(parts_gt[0], this.page);
			
			data_str = parts_gt[1];
		}

		return html;

	} // end of data_to_html
	
	// See function eleHandle()
	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

	// This editor_optn is called by class Editor.editor_open()
	editor_open() {
		// console.log("wc.js class SubsectionContent editor_open()");		

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorSubsectionContent);
		html = html.replace("<!--placeHolder-->", htmlEditorEnter);
		html = html.replace("<!--placeHolder-->", htmlEditorMove);

		let ele = this.page.editor().ele_setup(this, html);

		// this.isBlank means no subsectionContent exist.
		// so delite, move and insert are not applicable .
		if(this.isBlank){
			eleVisibleSet(ele, {
				'editor_delete' : 0
				,'editor_hr' : 0
				,'editor_moveMenu' : 0
				,'editor_insertMenu' : 0
			});
		}

		this.editorDataSet(ele);

		const eleDef = {
			"ele" : ele,
			"targetFcode" : "editorTarget",
		};

		if(this.isBlank){
			eleDef.targetFcode = "editor_insertTarget";
		}

		this.editorEle(eleDef);
		
	} // end of editor_open

	editorTarget() {
		return this.ele().drawn;
	} // end of editorTarget
	
	editorDataSet(ele) {
		// title,
		ele.querySelector(".title").innerHTML = this.subsection.data()["title"];
		
		ele.querySelector(".title").innerHTML = this.subsection.data()["title"];

		// type
		if(this.data()["type"]){
			let optionValueType = 'option[value="'+this.data()["type"]+'"]';
			ele.querySelector(optionValueType).selected = true;
		}
		
		// content
		let eleContent = ele.querySelector(".editor_subsection_content");
		eleContent.textContent = dataDecode(this.data()["value"]);
		
	} // end of editorDataSet
	
	editor_insertTarget() {
		if(this.insertTarget){
			return this.insertTarget.ele().drawn;
		}
		return this.subsection.contentBlank().ele().drawn;
	} // end of editor_insertTarget

 	editor_enter() {
		// console.log("wc.js class SubsectionContent editor_enter()");

		// type
		let select = 'select[name="editor_subsection_content_type"]';
	    const eleSelect = this.editorEle().drawn.querySelector(select);
	    const content_type = eleSelect.selectedOptions[0].value;
		this.data()["type"] = content_type;

		// value
	    let contentEle = this.editorEle().drawn.querySelector(
			".editor_subsection_content");

		// Whatever the type is, save as entered .
		this.data()["value"] = contentEle.value;
		
		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
		// a temporary SubsectionContent made at this.editor_insertMenu()
		if(this.isBlank){
			this.subsection.contentInsert(this);
			// this.subsection.content_list(undefined);
			// reflesh a whole of subsection
			// eleRefresh(this.subsection);
			this.subsection.eleDraw();

			// delet this self
			this.subsection.contentBlank(undefined);
			
		} else {
			// reflesh only this content
			// eleRefresh(this);
			this.eleDraw();
		}

	} // end of editor_enter

	editor_delete() {

		eleVisibleSet(
			// this.editor_ele(),
			this.editorEle().drawn,
			{
				'editor_enter' : 0
				, 'editor_cancel' : 0
				,'editor_delete' : 0
				,'editor_moveMenu' : 0
				,'editor_insertMenu' : 0
				,'editor_deleteConfirm' : 1
			}
		);
		
	} // end of editor_delete

	editor_deleteCancel() {

		this.page.editor().editor_close(this);
		return;
		
		eleVisibleSet(
			// this.editor_ele(),
			this.editorEle().drawn,
			{
				'editor_enter' : 1
				, 'editor_cancel' : 1
				,'editor_delete' : 1
				,'editor_deleteConfirm' : 0
			}
		);

	} // end of editor_deleteCancel

	editor_deleteExecute() {
		// console.log("wc.js class SubsectionContent editor_deleteExecute()");

		// make content data without the content to be deleted
		let content = [];
		let contents = this.subsection.contents();
		let ino;
		for(let i = 0; i < contents.length; i++){
			if(this == contents[i]){
				ino = i;
				break;
			}
			// except this.index_no to be deleted
			if(i == this.index_no){ continue;}
			// content.push(this.subsection.content(i).data());
			content.push(this.subsection.contents()[i].data());
		}

		if(ino == undefined){ return; }
		
		this.subsection.data().content.splice(ino, 1);

		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
		// reflesh subsection
		this.subsection.eleDraw();
		
	} // end of editor_deleteExecute

	editor_insertMenu() {
		// console.log("class SubsectionContent editor_insertMenu()");
		
		let contentNew =  this.subsection.contentBlank();
		contentNew.insertTarget = this;

		// This calls contentNew.editor_open()
		this.page.editor().editor_open(contentNew);

	} // end of editor_insertMenu
	
	editor_moveMenu(event) {

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorMoveTo);

		// editorMoveEventListenerSet(this, this.subsection.content_list());
		editorMoveEventListenerSet(this, this.subsection.contents());

		// let editor_ele = this.page.editor().ele_setup(this, html);
		let ele = this.page.editor().ele_setup(this, html);

		// this.editor_ele(editor_ele);
		this.editorEle({"ele" : ele});

		event.stopPropagation(); // prevent editor_move
		
	} // end of editor_moveMenu

	editorMoveMenuEventListenerRemove() {

		for(let contentTarget of this.subsection.contents()){
			let eleTarget = contentTarget.ele().drawn;
			eleTarget.removeEventListener(
				contentTarget.editorMoveOn.trigger,							
				contentTarget.editorMoveOn.action,
			);
		}

	} // end of editorMoveMenuEventListenerRemove

	editor_move(contentTarget, event) {
		
		let dataListNew = [];
		for(let contentCrt of this.subsection.contents()){

			// contentCrt.data() is to move
			if(contentCrt == this){
				// not move to itself
				if(contentTarget.data() != this.data()){
					continue;
				}
			}

			dataListNew.push(contentCrt.data());

			if(contentCrt == contentTarget){
				if(contentTarget != this){
					dataListNew.push(this.data());
				}
			}
		}

		this.subsection.data().content = dataListNew;

		this.editorMoveMenuEventListenerRemove();

		this.page.editor().editor_close();

		this.subsection.contents(undefined);

 		this.page.editor().menu().changed(true);

		// eleRefresh(this.subsection);
		this.subsection.eleDraw();

	} // end of editor_move

} // end of class SubsectionContent

function eleVisibleSet (eleArg, req) {
    // eleVisibleSet (eleArg, req);
    // req: {'class0', 1, 'class1': 0};
    // class0: class name;
    // 1: show, 0: off;
    if( ! eleArg){ return;}

    for(const key0 in req){
		let elePart = eleByClass(eleArg, key0);
		if( ! elePart){ continue;}
		if(req[key0]){
			elePart.classList.remove('invisible');
		}else{
			elePart.classList.add('invisible');
		}
    }
	
} // end of function eleVisibleSet

function eleByClass (eleRoot, className) {
    // ele1 = eleByClass(eleRoot, className);
    // find an element containing className;
    // from eleRoot and its childlen;
    
    let elePart = eleRoot.querySelector("."+className);
    if( ! elePart){
		if(eleRoot.classList.contains(className)){
			elePart = eleRoot;
		}
    }
    return elePart;
} // end of function eleByClass;

// convert text data to HTML.
// space and tab to <pre class="inline0">space and tab</pre>
// \n to <br>
// // \n\n to <p>a</p><p>b</p>
// content1\n\ncontent2 to  <p>contetnt1</p> <p>contetnt2</p>
const p_start = "<p>";
const p_end = "</p>";
function text_to_html2(text) {
	// console.log("wc.js function text_to_html2 text: " + text);

	// <>\n<> shild be handled as <><>
	// any space between > : end of tag and < : start of tag should be ignore


    text = text.replace(/[ \t]{2,}|\t+/g, '<pre class="inline0">$&</pre>');

	let html = p_start;
	let list = text.split("\n");
	while (0 < list.length) {
		let line = list.shift();
		html += line;
		// Next line exists means \n exists.
		if (0 < list.length) {
			// \n   : <br>
			// \n\n : <p></p>
			//
			// Next line is emply.
			// Means \n\n
			if (list[0].length == 0) {
				// Remove next one since empty.
				list.shift();
				html += (p_end + p_start);
			} else {
				html += "<br>"
			}
		}
	}

	html += p_end;
	
	return html;
	
} // end of function text_to_html2

function editorDataSet(eleTop, harg) {

	for(let key in harg){
		let ele = eleTop.querySelector("." + key);
		if(! ele){ continue;}

		if(ele.tagName == "INPUT"){ ele.value = harg[key];}
		if(ele.tagName == "SPAN"){
			ele.appendChild(document.createTextNode(harg[key]));
		}
	}

} // end of function editorDataSet

// Consider to send rev no .
// If rev no sent by posData and the rev no of file are not same,
// data conflict might be happen .
//
async function posData(req, data){
	// console.log("wc.js function posData");

	const response = await fetch(
		document.URL,
		{
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'wc-request' : req,
			},
			body: JSON.stringify(data),
		},
	)

	return response.json();

} // end of function posData

// When open url with hash eg. http://domain.com/abc.html#def
// the element with id "def" does not exists yet
// because it will be created on page.eleDraw() at funtion bodyOnload()
// So scroll to the element with id "def" after draw page elements.
function scrollUrlHash() {
	// console.log("wc.js function scrollUrlHash");

	let url = new URL(document.URL);
	
	let id = url.hash ? url.hash.slice(1) : undefined;

	scrollHash(id);

} // end of function scrollUrlHash

function scrollHash(id) {
	// console.log("wc.js function scrollHash");

	let eleTarget = id ? document.getElementById(id) : undefined;
	
	let targetRect = eleTarget ? eleTarget.getBoundingClientRect() : undefined;

	if(targetRect){
		window.scrollTo({
			left: targetRect.left,
			top: targetRect.top,
			behavior: 'smooth'
			// auto does not work
			// behavior: 'auto'
		});

		// These do not work
		// window.scrollTo(targetRect.left, targetRect.top);
		// window.scrollTo(10, 10);
		
	}
	
} // end of function scrollHash

class EditorPageMove {
	// console.log("wc.js class EditorPageMove");

	constructor(page) {
		this.page = page;
	} // end of constructor

	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

	// This editor_optn is called by class Editor::editor_open
	editor_open() {
		// console.log("wc.js class EditorPageMove editor_open");

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlEditorPageMove);
		html = html.replace("<!--placeHolder-->", htmlEditorEnter);
		
		let ele = this.page.editor().ele_setup(this, html);

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "editorTarget",
		};

		this.editorEle(eleDef);
		
	} // end of editor_open
	
	editorTarget() {
		return this.page.editor().menu().editorEle().drawn;
	} // end of editorTarget

 	editor_enter() {
		// console.log("wc.js class EditorPageMove editor_enter()");

		let editor_ele = this.editorEle().drawn;
	    let parentUrlEle = editor_ele.querySelector(".parentUrl");
		let parentUrl = parentUrlEle.value.trim();
		if(parentUrl.length == 0){ return; }
	    let destUrlEle = editor_ele.querySelector(".destUrl");
		let destUrl = destUrlEle.value.trim();
		if(destUrl.length == 0){ return; }
		
		let data = {"parent_url" : parentUrl, "dest_url" : destUrl};
		let res = posData("page_move", data);
		
		res.then(data => {
			console.log("wc.js class EditorPageMove editor_enter() response data.res:" + data.res);
		});

	} // end of editor_enter
	
} // end of class EditorPageMove

const htmlEditorPageMove = `
    <tr>
	  <td></td>
	  <td>Page Move</td>
	</tr>
    <tr>
	  <td></td>
	  <td>from the URL and its sub pages to this page</td>
	</tr>

    <tr>
	  <td>parent url</td>
	  <td><input class="parentUrl"></td>
    </tr>

    <tr>
	  <td>dest url</td>
	  <td><input class="destUrl"></td>
    </tr>
<!--placeHolder-->
`; // end of const htmlEditorPageMove

class HrefReference {
	// console.log("wc.js class HrefReference");

	constructor(page) {
		this.page = page;
	} // end of constructor

	data() {

		if(0 < arguments.length){
			this.dataObj = arguments[0];
			return this.dataObj;
		}

		if(this.dataObj){ return this.dataObj; }

		if(this.page.page_json["data"]["href_reference"] == undefined){
			this.page.page_json["data"]["href_reference"] = {};
		}
		
		let data = this.page.page_json["data"]["href_reference"];

		return this.data(data);
		
	} // end of data

	// See function eleHandle()
	editorEle(...args) {

		if(this.editorEleObj == undefined){ this.editorEleObj = {}; }
		return eleHandle(this, this.editorEleObj, ...args);

	} // end of editorEle

	editor_data_set() {
		// console.log("wc.js class HrefReference editor_data_set()");

		let editor_ele = this.editor_ele();
		let data = this.data();

		let value = "";
		for(let key in data){
			value += key + " " + data[key] + "\n";

			console.log("wc.js class HrefReference editor_data_set() key:" + key);
			
		}

		let ele;
	    ele = editor_ele.querySelector(".editor_href_reference");
		ele.value = value;

	} // end of editor_data_set
	
	// This editor_optn is called by class Editor::editor_open
	editor_open() {
		// console.log("wc.js class HrefReference editor_open");

		let html = htmlEditorBox;
		html = html.replace("<!--placeHolder-->", htmlHrefReferenceEditor);
		html = html.replace("<!--placeHolder-->", htmlEditorEnter);
		let ele = this.page.editor().ele_setup(this, html);

		let eleDef = {
			"ele" : ele,
			"targetFcode" : "editorTarget",
		};

		// value set
		let data = [];
		let keys = Object.keys(this.data());
		let thisData = this.data();
		Object.keys(thisData).forEach(
			function(key){ data.push(key + " " + thisData[key]);}
		);
		ele.querySelector(".editor_href_reference").value = data.join("\n");

		this.editorEle(eleDef);

	}

	editorTarget() {
		return this.page.editor().menu().editorEle().drawn;
	} // end of editorTarget

 	editor_enter() {
		// console.log("wc.js class HrefReference editor_enter");

	    let ele = this.editorEle().drawn.querySelector(".editor_href_reference");
		
		let data = this.data();
		
		for(let line of ele.value.split("\n")){
			if(line.length == 0){ continue; }
			let v = line.split(" ");
			if(v.length == 0){ continue; }
			data[v[0]] = v[1];

		}
		
		this.page.editor().editor_changed(true);
		this.page.editor().editor_close(this);
		
		
	} // end of editor_enter
	
} // end of class HrefReference

const htmlHrefReferenceEditor = `
    <tr>
	  <td></td>
	  <td>Href Reference</td>
	</tr>
	<tr>
	  <td></td>
	  <td><textarea class="editor_href_reference"></textarea></td>
	</tr>
<!--placeHolder-->
`; // end of const htmlHrefReferenceEditor

function dataDecode(str) {

	if(str == undefined){ return;}

	// Entity references are converted at function page_json_read
	// So comment out  str = entityReferenceUnset(str);
	// str = entityReferenceUnset(str);
	
	str.replaceAll('\"', '"');

	return str;
	
} // end of function dataDecode

function dataEncode_(str) {

	// escape "
	str = str.replace(/"/g, '\"');

	str = entityReferenceSet(str);
	
	return str;

} // end of function dataEncode_

// Convert
// \<: &lt;,
// \>: &gt;,
// But considering \\, eg \\< is not converted .
function textAngleToEntity(str) {
	// console.log("wc.js function textAngleToEntity");

	return str.replace(textAngleRegex, textAngleToEntityReplacer);

} // end of function textAngleToEntity

// const textAngleRegex = /(\\*)([<|>])/;
const textAngleRegex = /(\\*)((?:\\<)|(?:\\>))/g;
function textAngleToEntityReplacer() {

	// arguments[0]: hole of the match
	// arguments[1]: \*
	// arguments[2]: \<|\>
	// arguments[]: 
	// arguments[]:

	// arguments[2] is with \ (\< or \>) ,
	// so if arguments[1].length is odd,
	// that means numbers of \ in arguments[1] + arguments[2] is even .
	// even: 1 means \\< , \\ (escaped \) and <
	// even: 0 means \<  , < is escaped by \ (\<)
	let even = arguments[1].length % 2;

	if(even){ return arguments[0]; }

	if(arguments[2] == "\\<"){ return arguments[1] + "&lt;"; }
	
	if(arguments[2] == "\\>"){return arguments[1] + "&gt;";}	
	
	return arguments[0];

} // end of function textAngleToEntityReplacer

function bodyOnloadFetch () {
	// console.log('wc.js bodyOnloadFetch ');

	let req = "onload";
	let body_contents = "content_data";


	fetch(
		document.URL,
		{
			method: 'POST',
			headers: {
				'Content-Type': 'text/plain',
				'wc-request' : req,
			},
			body: body_contents,
		})

		.then(response =>
			  // response.json()
			  // response.ok
			  // "response"
			  // response.body
			  response.text()
			 )
	
		.then(data => {
			// pageJsonMonitor(data);
			// pageUpdate(data);
			console.log('bodyOnloadFetch Success:', data);
		})
		.catch((error) => {
			console.error('bodyOnloadFetch Error', error);
		})
	;

} // end of function bodyOnloadFetch

function fetchDataTest01 () {

	let req = "test01";
	let page_json = {"data" : "<>&\""};
	let body_contents = JSON.stringify(page_json);

	fetch(
		document.URL,
		{
			method: 'POST',
			headers: {
				'Content-Type': 'text/plain',
				'wc-request' : req,
			},
			body: body_contents,
		})
		.then(response =>
			  // response.json()
			  response.text()
			 )
	
		.then(data => {
			test01_jsset(data);
		})
		.catch((error) => {
			console.error('fetchDataTest01 Error', error);
		})
	;
	
} // end of function fetchDataTest01

let test01_json;

function test01_jsset(data) {

	let f0 = new Function("return " + data + ";");

	test01_json = f0();

	console.log('test01_jsset.foo :', test01_json.foo);

	// <script type="text/javascript" class="page_json">
	let script_ele = document.createElement('script');
	script_ele.setAttribute('type', "text/javascript");
	script_ele.setAttribute('class', "page_json");

	script_ele.innerHTML = "function test01_f () { console.log('test01_f done');}";

	let head = document.getElementsByTagName("head");
	head[0].appendChild(script_ele);

	test01_f();

}


const regx_tag_href = /(##.+?\$)/;
const regx_href_value = /href=(["'])([^\1]+?)(?<!\\)\1/;
const regx_href = /href\s*=\s*(["'])([^\1]+?)(?<!\\)\1/;

// for memo;
// const subA = subsectionTop.querySelector("a[name="+this.hrefName()+"]");

// Split str_arg at "<" to two parts and
// return [part_first, part_rest]
// part_first: before "<"
// part_rest : "<" and the after
//
// To use charactor backslash \ in text, use "\\".
// Because \ is used to escape some.
//
// Local Roles
// <, and > ("<", ">") are for HTML tags.
// \< and \> ("\\<", "\\>") are considered as text < and >.
// If "\<" could be used as escape sequence, it did.
// Insted, consider caractor \ and < ("\\<") as text <.
// \\ ("\\\\") is text \.
//
// ex. abc<def to (abc, def)
// ex. abc\<def ("abc\\<def") to (abc\<def,)
// ex. abc\\<def ("abc\\\\<def") to (abc\\, def)
//
// [\\] means single back slash charactor
//
// [^<]* try to take the longest, that contains [\\] as well.
// [^<]*? try to take the shortest, that does not contains [\\],
// so [\\]* contains the logest continuous \\ strings.
const regx_lt = /^([^<]*?)([\\]*)<(.*)$/;
function divide_by_lt(str_arg) {
	// console.log("wc.js function divide_by_lt str_arg: " + str_arg);

	let res = str_arg.match(regx_lt);
	// Not match
	if (res == undefined) { return [str_arg, ""]; }
	// res[0] : all match parts
	// res[1] : before [\\]*
	// res[2] : [\\]*
	// res[3] : after <

	let part_first = res[1];
	const part_slash = res[2];
	let part_rest = res[3];

	// console.log("wc.js function divide_by_lt part_first: " + part_first);
	// console.log("wc.js function divide_by_lt part_slash: " + part_slash);
	// console.log("wc.js function divide_by_lt part_rest: " + part_rest);
	
	// number of double backslashes \\
	let bs_times = Math.floor(part_slash.length / 2);
	// console.log("wc.js function divide_by_lt bs_times: " + bs_times);
	// number of escape(/), 0 or 1
	let es_time = part_slash.length % 2;
	// console.log("wc.js function divide_by_lt es_time: " + es_time);

	// Push back double backslash \\ ("\\\\") 
	for (let i=0; i<bs_times; i++) {
		part_first += "\\\\";
	}
	// console.log("wc.js function divide_by_lt part_first: " + part_first);
	
	// \< is escaped <, it is text, not html element part.
	// Push back \< ("\\<")
	// That mesn < for html element was not found,
	// try to find next <, recursively.
	if (es_time == 1) {
		part_first += "\\<";
		
		// console.log("wc.js function divide_by_lt part_rest: " + part_rest);
		
		// Find < recursively
		let res2 = divide_by_lt(part_rest);
		part_first += res2[0];
		part_rest = res2[1];
	} else {
		// part_rest is not undefined.
		// In case part_rest == undefined,
		// it was returned at if (res == undefined)
		part_rest = "<" + part_rest
	}

	// console.log("wc.js function divide_by_lt return: " + [part_first, part_rest]);
	return [part_first, part_rest];
	
} // end of function divide_by_lt

// divide by ">"
const regx_gt = /^([^>]*?)([\\]*)>(.*)$/;
function divide_by_gt(str_arg) {
	// console.log("wc.js function divide_by_gt str_arg: " + str_arg);

	let res = str_arg.match(regx_gt);
	// console.log("wc.js function divide_by_gt res: " + res);
	// Not match
	if (res == undefined) {
		// console.log("wc.js function divide_by_gt return: " + [str_arg]);
		return [str_arg, ""]; }
	// res[0] : all match parts
	// res[1] : before [\\]*
	// res[2] : [\\]*
	// res[3] : after >

	let part_first = res[1];
	const part_slash = res[2];
	let part_rest = res[3];

	// console.log("wc.js function divide_by_gt part_first: " + part_first);
	// console.log("wc.js function divide_by_gt part_slash: " + part_slash);
	// console.log("wc.js function divide_by_gt part_rest: " + part_rest);
	
	// number of double backslashes \\
	let bs_times = Math.floor(part_slash.length / 2);
	// console.log("wc.js function divide_by_gt bs_times: " + bs_times);
	// number of escape(/), 0 or 1
	let es_time = part_slash.length % 2;
	// console.log("wc.js function divide_by_gt es_time: " + es_time);

	// Push back double backslash \\ ("\\\\") 
	for (let i=0; i<bs_times; i++) {
		part_first += "\\\\";
	}
	// console.log("wc.js function divide_by_gt part_first: " + part_first);
	
	// \> is escaped >, it is text, not html element part.
	// Push back \> ("\\>")
	// That mesn > for html element was not found,
	// try to find next >, recursively.
	if (es_time == 1) {
		part_first += "\\>";
		
		// console.log("wc.js function divide_by_gt part_rest: " + part_rest);
		
		// Find > recursively
		let res2 = divide_by_gt(part_rest);
		part_first += res2[0];
		part_rest = res2[1];
	} else {
		// part_rest is not undefined.
		// In case part_rest == undefined,
		// it was returned at if (res == undefined)
		
		// part_rest = ">" + part_rest
		part_first += ">";
	}

	// console.log("wc.js function divide_by_gt return: " + [part_first, part_rest]);
	return [part_first, part_rest];
	
} // end of function divide_by_gt

/*
const emplty_element_name_list = ["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"];
 */

function preventContextmenu (event) {
	// stop default right click action;
	event.preventDefault();
} // end of function preventContextmenu;

function test02() {
	// console.log("wc.js function test02");

	let str = "<>\n<>";
	// <><>

	console.log("wc.js function test02 str: " + str);

	str = str.replace(/>\s+</, "><");

	console.log("wc.js function test02 str: " + str);
	
} // end of function test02
