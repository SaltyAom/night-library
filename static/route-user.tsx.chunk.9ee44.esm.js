(window.webpackJsonp=window.webpackJsonp||[]).push([[8],{l6NR:function(t,e,c){"use strict";c.r(e);var i=c("hosL"),a=c("QRet"),o=c("QTFd"),s=c("MRB6");c("qIS1"),e.default=()=>{let{dispatch:t}=Object(s.b)(),[e,c]=Object(a.h)(()=>!0,!1),{username:n,initial:b}=Object(s.b)("username","initial");return e?Object(i.h)(o.b,{href:"/"}):n||b?Object(i.h)("section",{id:"user"},Object(i.h)("h1",{class:"title"},"Welcome back ",n),Object(i.h)("aside",{class:"action-list"},Object(i.h)(o.a,{href:"/my-book"},Object(i.h)("a",{class:"action"},"My Book")),Object(i.h)("button",{class:"action",onClick:async()=>{await fetch("http://localhost/logout",{method:"GET",credentials:"include"}),t("LOGOUT",""),c(1)}},"Logout"))):Object(i.h)(o.b,{href:"/signin"})}},qIS1:function(){}}]);