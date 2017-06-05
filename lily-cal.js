const fs = require('fs');
const moment = require('moment');
require('moment-recur');
var icalToolkit = require('ical-toolkit');
var express = require('express');
//Create a builder
var builder = icalToolkit.createIcsFileBuilder();

/*
 * Settings (All Default values shown below. It is optional to specify)
 * */
builder.spacers = true; //Add space in ICS file, better human reading. Default: true
builder.NEWLINE_CHAR = '\r\n'; //Newline char to use.
builder.throwError = true; //If true throws errors, else returns error when you do .toString() to generate the file contents.
builder.ignoreTZIDMismatch = true; //If TZID is invalid, ignore or not to ignore!

//Name of calander 'X-WR-CALNAME' tag.
builder.calname = 'Lily Cal';
//Cal timezone 'X-WR-TIMEZONE' tag. Optional. We recommend it to be same as tzid.
builder.timezone = 'Europe/London';
//Time Zone ID. This will automatically add VTIMEZONE info.
builder.tzid = 'Europe/London';

//Method
builder.method = 'REQUEST';

const caleventdates = moment('04-12-2017').recur( moment('04-12-2017').add(1, 'year') ).every(1).weeks();

caleventdates.all().map(eventFactory).map(event => builder.events.push(event));

function eventFactory(date, i) {
  return {
    start: date.toDate(),
    end: date.toDate(),
    transp: 'TRANSP',
    allDay: true,
    summary: `Lily is ${i} weeks old`,
    mehtod: 'REQUEST'
  }
}

// const birthday = '04-12-2017';
// console.log('starting from ', moment(birthday).toDate());
// cal.addP rop('DTSTART', new Date('2017-04-12 10:00:00'))
// cal.addProp('DTEND', new Date('2017-04-12 10:00:00'))


// const weeks = [];
// for (var index = 1; index <= 52; index++) {
//   const thisWeek = moment(birthday).add(1, 'weeks')
//   console.error('Adding cal event for wk', thisWeek.toDate());
//   weeks.push(thisWeek);
// }

// weeks.reduce(function (eventDate, ageInWeeks) {
//   return const cal = new ICS.VEVENT();
//     start: eventDate.format('YYYY-MM-DD'),
//     end: eventDate.format('YYYY-MM-DD'),
//     title:'Lily '+ageInWeeks+'wk old'
//   })
// }, cal).join('');

// fs.writeFileSync('lilycal.ics', builder.toString());


const server = express();

server.get('/lily.ics', function (req, res) {
  res.send(builder.toString()).status(200);
}).listen(1337, function () {
  console.log('started: 1337')
});