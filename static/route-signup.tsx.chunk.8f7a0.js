(window.webpackJsonp=window.webpackJsonp||[]).push([[7],{HWfQ:function(){},ILCp:function(e,n,t){"use strict";t.r(n);var r=t("hosL"),c=t("QRet"),a=t("QTFd");t("HWfQ");n.default=function(){var e=Object(c.j)(""),n=e[0],t=e[1],s=Object(c.h)((function(){return!0}),!1),o=s[0],i=s[1],u=Object(c.i)(""),l=Object(c.i)(""),h=Object(c.i)("");Object(c.d)((function(){fetch("http://localhost/api/refresh",{method:"POST",credentials:"include"}).then((function(e){return e.text()})).then((function(e){e&&i(1)}))}),[]);var d=Object(c.a)((function(e){u.current=e.currentTarget.value}),[]),b=Object(c.a)((function(e){l.current=e.currentTarget.value}),[]),p=Object(c.a)((function(e){h.current=e.currentTarget.value}),[]),f=Object(c.a)((function(e){return new Promise((function(n,r){var c;return e.preventDefault(),u.current&&l.current?l.current!==h.current?n(t("Password doesn't match")):Promise.resolve(fetch("http://localhost/register",{method:"POST",headers:{"Content-Type":"application/x-www-form-urlencoded;charset=UTF-8"},credentials:"include",body:new URLSearchParams({username:u.current,password:l.current})}).then((function(e){return e.json()}))).then((function(e){try{return(c=e).success||"Already signin in"===c.info?i(1):t(c.info),n()}catch(e){return r(e)}}),r):n(t("Username and Password can't be blank"))}))}),[]);return Object(r.h)("form",{id:"login",action:"http://localhost/register",method:"POST",onSubmit:f},Object(r.h)("h1",{class:"title"},"Signin"),Object(r.h)("label",{class:"label",for:"username"},"Username"),Object(r.h)("input",{class:"input username",name:"username",type:"text",placeholder:"Username",onInput:d}),Object(r.h)("label",{class:"label",for:"password"},"Password"),Object(r.h)("input",{class:"input password",name:"password",type:"password",placeholder:"Password",onChange:b}),Object(r.h)("label",{class:"label",for:"confirm-password"},"Confirm Password"),Object(r.h)("input",{class:"input confirm-password",name:"confirm-password",type:"password",placeholder:"Confirm Password",onChange:p}),n&&Object(r.h)("p",{class:"warning -hidden"},n),Object(r.h)("section",{class:"action"},Object(r.h)("button",{class:"login"},"Signup"),Object(r.h)(a.a,{href:"/signin"},Object(r.h)("a",{class:"link"},"Signin"))),o&&Object(r.h)(a.b,{href:"/"}))}}}]);
//# sourceMappingURL=route-signup.tsx.chunk.8f7a0.js.map