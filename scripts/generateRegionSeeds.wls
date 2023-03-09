#!/usr/bin/env wolframscript
(* ::Package:: *)

{n, out} = $ScriptCommandLine[[{2,3}]];
Export[out, SpherePoints[Interpreter["Integer"]@n]]
