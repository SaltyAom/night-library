(window.webpackJsonp=window.webpackJsonp||[]).push([[4],{"kLe+":function(e,t,n){"use strict";function o(){return(o=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var o in n)Object.prototype.hasOwnProperty.call(n,o)&&(e[o]=n[o])}return e}).apply(this,arguments)}n.r(t);var r=n("hosL"),c=n("QRet"),i=n("QTFd"),s=n("mdnY"),a=n("LT/T"),l=n("GNgh"),h=(n("tyAA"),n("MRB6"));t.default=function(){var e=Object(h.b)("username","initial"),t=e.username,n=e.initial,b=Object(s.a)(["listBorrowed","{\n              listBorrowed {\n                data {\n                  id,\n                  title,\n                  author,\n                  price\n                }\n              }\n            }"],a.a),u=b.data,d=b.error,j=Object(c.j)([]),O=j[0],f=j[1],k=function(e){return new Promise((function(t,n){return Promise.resolve(Object(a.b)("returnBooks",'mutation {\n              returnBooks(books: ["'+e+'"]) {\n                success\n              }\n            }')).then((function(o){try{return o.success?(f([].concat(O,[e])),t()):t(console.log("Something went wrong"))}catch(e){return n(e)}}),n)}))};return d?Object(r.h)("section",{id:"book-list",class:"-error"},Object(r.h)("h3",{class:"title"},"Failed to fetch"),Object(r.h)("p",{class:"detail"},"There is a problem requesting an list of books."),Object(r.h)("button",{class:"action",onClick:function(){return window.location.reload()}},"Retry")):!u||n?Object(r.h)("section",{id:"book-list"},Object(r.h)("h1",{class:"header"},"My Book"),Array(8).fill(null).map((function(e,t){return Object(r.h)(l.a,{key:t,preload:!0,hiddenPrice:!0})}))):t?u.length&&u.length!==O.length?Object(r.h)("section",{id:"book-list"},Object(r.h)("h1",{class:"header"},"My Book"),u.filter((function(e){return!O.includes(e.id)})).map((function(e){return Object(r.h)(l.a,o({key:e.id},{book:e},{hiddenPrice:!0,onClick:k,action:"Return"}))}))):Object(r.h)("section",{id:"book-list",class:"-error"},Object(r.h)("h3",{class:"title"},"No Borrow"),Object(r.h)("p",{class:"detail"},"No book have been borrowed."),Object(r.h)(i.a,{href:"/"},Object(r.h)("button",{class:"action -compact"},"Borrow some book"))):Object(r.h)(i.b,{href:"/signin"})}}}]);
//# sourceMappingURL=route-my-book.tsx.chunk.70ba5.js.map