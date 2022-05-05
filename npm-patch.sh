#!/bin/sh
npm version patch
git push origin HEAD --follow-tags
