/**
 * data.js — Static data layer for Cube Games 2026
 * Source: feed_entries.csv (April 17, 2026 capture)
 *
 * To update: edit TEAM_ROSTER (teams + members) and ACTIVITY_SCORES (challenge results) directly.
 */

(function (global) {
  'use strict';

  // ═══════════════════════════════════════════════════════════════════
  // TEAM ROSTER — edit here to change team names or member assignments
  // ═══════════════════════════════════════════════════════════════════
  var TEAM_ROSTER = {
    'Vortex Velocity': [
      'Zee Ph', 'Robin BongCaiTim', 'Wiramon TK', 'Fazalika Hismawan', 'Marvin Cho', 'Sarabjit Singh',
    ],
    'Neon Stride': [
      'yiggythestroller 🐬', 'tanin padungkirtsakul', 'Suracheth Khaewklam', 'Nont Fakungkun', 'Lognath R C', 'mncy snoopy',
    ],
    'Apex Pulse': [
      'Mingkwan Senavat', 'Pawitra Chatrittichaikul', 'Pratishtha Kohli', 'Nikunta Piyaket', 'Eh Apinya', 'Khoi Nguyen',
    ],
    'Thunder Pace': [
      'Pitchaya Vasoontararat', 'Nipphit Apisitpuwakul', 'Ben Thongchai', 'Book Teeranai', 'Patrick Ocampo', 'Robbuckets s',
    ],
    'Solar Sprint': [
      'Salisa S', 'Jidapa Chaovanapricha', 'Diksha Bhati', 'Nirali Singh', 'Francisca Suryawijaya',
    ],
    'Kinetic Edge': [
      'Win Chanchai', 'Lapid Pongcharoenyong', 'Peem Benjasiriwan', 'Nico Revaldo', 'Nora Phung',
    ],
  };

  // ═══════════════════════════════════════════════════════════════════
  // RAW FEED ENTRIES  [id, athlete_name, distance_km, date_label]
  // "Today"     = April 17, 2026  |  "Yesterday" = April 16, 2026
  // ═══════════════════════════════════════════════════════════════════
  var RAW = [
    [1776393806, 'Win Chanchai',               0.31, 'Today'],
    [1776391266, 'Lapid Pongcharoenyong',       2.60, 'Today'],
    [1776388788, 'Mingkwan Senavat',            4.66, 'Today'],
    [1776358261, 'Pratishtha Kohli',            0.65, 'Yesterday'],
    [1776357272, 'yiggythestroller 🐬',         0.82, 'Yesterday'],
    [1776350442, 'Pawitra Chatrittichaikul',    2.02, 'Yesterday'],
    [1776350289, 'Peem Benjasiriwan',           5.00, 'Yesterday'],
    [1776348004, 'yiggythestroller 🐬',         1.04, 'Yesterday'],
    [1776347586, 'Pawitra Chatrittichaikul',    1.02, 'Yesterday'],
    [1776346193, 'Eh Apinya',                   0.80, 'Yesterday'],
    [1776343883, 'Sarabjit Singh',              3.19, 'Yesterday'],
    [1776343522, 'tanin padungkirtsakul',       0.45, 'Yesterday'],
    [1776343373, 'Suracheth Khaewklam',         3.89, 'Yesterday'],
    [1776342961, 'Robin BongCaiTim',           10.02, 'Yesterday'],
    [1776341045, 'yiggythestroller 🐬',         4.01, 'Yesterday'],
    [1776340958, 'Pitchaya Vasoontararat',      2.70, 'Yesterday'],
    [1776340903, 'tanin padungkirtsakul',       0.37, 'Yesterday'],
    [1776340765, 'Jidapa Chaovanapricha',       0.55, 'Yesterday'],
    [1776340473, 'Salisa S',                    2.01, 'Yesterday'],
    [1776326443, 'Zee Ph',                      1.22, 'Yesterday'],
    [1776325933, 'Salisa S',                    1.18, 'Yesterday'],
    [1776318043, 'Mingkwan Senavat',            0.93, 'Yesterday'],
    [1776317164, 'Zee Ph',                      2.10, 'Yesterday'],
    [1776314755, 'tanin padungkirtsakul',       0.33, 'Yesterday'],
    [1776314038, 'Suracheth Khaewklam',         0.77, 'Yesterday'],
    [1776304168, 'tanin padungkirtsakul',       6.31, 'Yesterday'],
    [1776275928, 'yiggythestroller 🐬',         2.73, 'April 15, 2026'],
    [1776266995, 'yiggythestroller 🐬',         2.05, 'April 15, 2026'],
    [1776255296, 'Diksha Bhati',                1.05, 'April 15, 2026'],
    [1776254897, 'Pawitra Chatrittichaikul',    3.01, 'April 15, 2026'],
    [1776254793, 'Nikunta Piyaket',             3.56, 'April 15, 2026'],
    [1776254160, 'Zee Ph',                      1.26, 'April 15, 2026'],
    [1776253929, 'Marvin Cho',                  9.26, 'April 15, 2026'],
    [1776231948, 'Suracheth Khaewklam',         1.26, 'April 15, 2026'],
    [1776219159, 'Zee Ph',                      0.73, 'April 15, 2026'],
    [1776214275, 'Wiramon TK',                  5.56, 'April 15, 2026'],
    [1776180650, 'Suracheth Khaewklam',         2.42, 'April 14, 2026'],
    [1776179652, 'yiggythestroller 🐬',         5.61, 'April 14, 2026'],
    [1776172987, 'Pratishtha Kohli',            1.28, 'April 14, 2026'],
    [1776172858, 'Pawitra Chatrittichaikul',    3.00, 'April 14, 2026'],
    [1776172847, 'Khoi Nguyen',                 3.38, 'April 14, 2026'],
    [1776172127, 'Pratishtha Kohli',            3.05, 'April 14, 2026'],
    [1776169679, 'tanin padungkirtsakul',       0.36, 'April 14, 2026'],
    [1776168952, 'Eh Apinya',                   3.29, 'April 14, 2026'],
    [1776168710, 'Nikunta Piyaket',             5.72, 'April 14, 2026'],
    [1776167841, 'Nipphit Apisitpuwakul',       1.16, 'April 14, 2026'],
    [1776167645, 'Zee Ph',                      2.13, 'April 14, 2026'],
    [1776167402, 'Robin BongCaiTim',            8.02, 'April 14, 2026'],
    [1776164604, 'Salisa S',                    1.96, 'April 14, 2026'],
    [1776154469, 'Zee Ph',                      6.54, 'April 14, 2026'],
    [1776154461, 'Suracheth Khaewklam',         1.51, 'April 14, 2026'],
    [1776141731, 'Ben Thongchai',               0.50, 'April 14, 2026'],
    [1776131760, 'Wiramon TK',                  8.81, 'April 14, 2026'],
    [1776114174, 'yiggythestroller 🐬',         1.11, 'April 13, 2026'],
    [1776095883, 'Suracheth Khaewklam',         1.99, 'April 13, 2026'],
    [1776091940, 'Robin BongCaiTim',            6.02, 'April 13, 2026'],
    [1776090447, 'Marvin Cho',                  3.00, 'April 13, 2026'],
    [1776088672, 'Suracheth Khaewklam',         1.38, 'April 13, 2026'],
    [1776082515, 'Francisca Suryawijaya',       3.67, 'April 13, 2026'],
    [1776079293, 'Eh Apinya',                   0.57, 'April 13, 2026'],
    [1776049808, 'Mingkwan Senavat',            1.73, 'April 13, 2026'],
    [1776045417, 'tanin padungkirtsakul',       6.03, 'April 13, 2026'],
    [1776042830, 'Wiramon TK',                  1.29, 'April 13, 2026'],
    [1776042205, 'Pawitra Chatrittichaikul',    1.58, 'April 13, 2026'],
    [1776041791, 'Wiramon TK',                  5.38, 'April 13, 2026'],
    [1776034195, 'Mingkwan Senavat',            4.81, 'April 13, 2026'],
    [1776019404, 'yiggythestroller 🐬',         5.61, 'April 12, 2026'],
    [1776014935, 'yiggythestroller 🐬',         3.92, 'April 12, 2026'],
    [1776008652, 'Suracheth Khaewklam',         0.69, 'April 12, 2026'],
    [1776000289, 'Eh Apinya',                   2.50, 'April 12, 2026'],
    [1775999246, 'yiggythestroller 🐬',         0.54, 'April 12, 2026'],
    [1775996938, 'Lognath R C',                 1.11, 'April 12, 2026'],
    [1775996464, 'Nont Fakungkun',              1.21, 'April 12, 2026'],
    [1775995036, 'Nipphit Apisitpuwakul',       1.00, 'April 12, 2026'],
    [1775994773, 'Nont Fakungkun',              5.05, 'April 12, 2026'],
    [1775994113, 'Nipphit Apisitpuwakul',       3.00, 'April 12, 2026'],
    [1775993930, 'Lognath R C',                 0.37, 'April 12, 2026'],
    [1775988108, 'Eh Apinya',                   0.69, 'April 12, 2026'],
    [1775984570, 'mncy snoopy',                 2.18, 'April 12, 2026'],
    [1775977618, 'Lognath R C',                 2.26, 'April 12, 2026'],
    [1775974047, 'Suracheth Khaewklam',         0.79, 'April 12, 2026'],
    [1775972054, 'Suracheth Khaewklam',         1.05, 'April 12, 2026'],
    [1775969628, 'Suracheth Khaewklam',         0.76, 'April 12, 2026'],
    [1775963831, 'Ben Thongchai',               0.43, 'April 12, 2026'],
    [1775959942, 'Zee Ph',                      1.60, 'April 12, 2026'],
    [1775958192, 'Nico Revaldo',                3.39, 'April 12, 2026'],
    [1775957155, 'Mingkwan Senavat',            0.83, 'April 12, 2026'],
    [1775954481, 'Fazalika Hismawan',          11.72, 'April 12, 2026'],
    [1775953621, 'Nico Revaldo',                1.04, 'April 12, 2026'],
    [1775951938, 'Zee Ph',                      3.83, 'April 12, 2026'],
    [1775947004, 'Mingkwan Senavat',            4.61, 'April 12, 2026'],
    [1775926547, 'Lognath R C',                 0.55, 'April 11, 2026'],
    [1775922000, 'Robin BongCaiTim',            4.50, 'April 11, 2026'],
    [1775917595, 'Wiramon TK',                  2.50, 'April 11, 2026'],
    [1775914562, 'Marvin Cho',                  6.30, 'April 11, 2026'],
    [1775910805, 'Marvin Cho',                  0.10, 'April 11, 2026'],
    [1775909869, 'Sarabjit Singh',              6.38, 'April 11, 2026'],
    [1775909533, 'Nipphit Apisitpuwakul',       2.67, 'April 11, 2026'],
    [1775909483, 'Pitchaya Vasoontararat',      5.94, 'April 11, 2026'],
    [1775909015, 'Nont Fakungkun',              4.21, 'April 11, 2026'],
    [1775908250, 'Marvin Cho',                  0.04, 'April 11, 2026'],
    [1775906125, 'Nont Fakungkun',              1.83, 'April 11, 2026'],
    [1775903967, 'Khoi Nguyen',                 7.49, 'April 11, 2026'],
    [1775889370, 'Lognath R C',                 0.32, 'April 11, 2026'],
    [1775884810, 'Lognath R C',                 0.41, 'April 11, 2026'],
    [1775872348, 'Wiramon TK',                  5.20, 'April 11, 2026'],
    [1775869117, 'Zee Ph',                     16.02, 'April 11, 2026'],
    [1775868666, 'tanin padungkirtsakul',       7.33, 'April 11, 2026'],
    [1775867223, 'Robin BongCaiTim',            8.15, 'April 11, 2026'],
    [1775866940, 'Fazalika Hismawan',           9.60, 'April 11, 2026'],
    [1775860142, 'Zee Ph',                      0.46, 'April 11, 2026'],
    [1775838117, 'Lognath R C',                 1.15, 'April 10, 2026'],
    [1775829457, 'Pawitra Chatrittichaikul',    1.62, 'April 10, 2026'],
    [1775829362, 'Eh Apinya',                   1.39, 'April 10, 2026'],
    [1775829170, 'Book Teeranai',               0.27, 'April 10, 2026'],
    [1775829080, 'Nirali Singh',                2.70, 'April 10, 2026'],
    [1775825507, 'Sarabjit Singh',              5.34, 'April 10, 2026'],
    [1775824185, 'Book Teeranai',               1.18, 'April 10, 2026'],
    [1775824052, 'yiggythestroller 🐬',         6.00, 'April 10, 2026'],
    [1775822959, 'Robin BongCaiTim',            3.08, 'April 10, 2026'],
    [1775821289, 'Book Teeranai',               0.79, 'April 10, 2026'],
    [1775806797, 'Eh Apinya',                   1.87, 'April 10, 2026'],
    [1775803786, 'Book Teeranai',               2.13, 'April 10, 2026'],
    [1775791623, 'Suracheth Khaewklam',         1.21, 'April 10, 2026'],
    [1775790455, 'Mingkwan Senavat',            0.94, 'April 10, 2026'],
    [1775747206, 'Suracheth Khaewklam',         1.55, 'April 9, 2026'],
    [1775742564, 'Robin BongCaiTim',            5.00, 'April 9, 2026'],
    [1775741567, 'Pitchaya Vasoontararat',      5.60, 'April 9, 2026'],
    [1775737258, 'Suracheth Khaewklam',         2.39, 'April 9, 2026'],
    [1775735866, 'yiggythestroller 🐬',         2.02, 'April 9, 2026'],
    [1775733553, 'Patrick Ocampo',              3.43, 'April 9, 2026'],
    [1775700419, 'Ben Thongchai',               0.30, 'April 9, 2026'],
    [1775700314, 'Mingkwan Senavat',            1.39, 'April 9, 2026'],
    [1775698330, 'Mingkwan Senavat',            2.63, 'April 9, 2026'],
    [1775697086, 'tanin padungkirtsakul',       6.33, 'April 9, 2026'],
    [1775695883, 'Zee Ph',                      4.16, 'April 9, 2026'],
    [1775657482, 'Suracheth Khaewklam',         1.78, 'April 8, 2026'],
    [1775654086, 'Nont Fakungkun',              7.57, 'April 8, 2026'],
    [1775653866, 'Zee Ph',                     10.19, 'April 8, 2026'],
    [1775651650, 'mncy snoopy',                 5.14, 'April 8, 2026'],
    [1775650795, 'Robbuckets s',                2.48, 'April 8, 2026'],
    [1775649699, 'Suracheth Khaewklam',         2.85, 'April 8, 2026'],
    [1775649597, 'Robbuckets s',                2.38, 'April 8, 2026'],
    [1775606781, 'Mingkwan Senavat',            0.97, 'April 8, 2026'],
    [1775440585, 'Zee Ph',                      3.50, 'April 6, 2026'],
    [1775407272, 'Book Teeranai',               1.27, 'April 5, 2026'],
    [1775386134, 'yiggythestroller 🐬',         4.09, 'April 5, 2026'],
    [1775351206, 'Mingkwan Senavat',            2.59, 'April 5, 2026'],
    [1775347877, 'Sarabjit Singh',              9.95, 'April 5, 2026'],
    [1775307987, 'Sarabjit Singh',              5.19, 'April 4, 2026'],
    [1775306534, 'yiggythestroller 🐬',         0.99, 'April 4, 2026'],
    [1775302879, 'Suracheth Khaewklam',         2.79, 'April 4, 2026'],
    [1775276148, 'Mingkwan Senavat',            2.92, 'April 4, 2026'],
    [1775265262, 'Zee Ph',                     16.01, 'April 4, 2026'],
    [1775228684, 'Suracheth Khaewklam',         2.66, 'April 3, 2026'],
    [1775224814, 'Nora Phung',                  6.33, 'April 3, 2026'],
  ];

  // ═══════════════════════════════════════════════════════════════════
  // DATE RESOLUTION
  // ═══════════════════════════════════════════════════════════════════
  var CY = 2026, CM = 3, CD = 17; // month 0-indexed (April = 3)
  var GAME_START = new Date(2026, 3, 8); // April 8, 2026 — competition start (month 0-indexed)

  // ═══════════════════════════════════════════════════════════════════
  // CHALLENGE ACTIVITY SCORES — filled in by organizer
  // ═══════════════════════════════════════════════════════════════════
  var ACTIVITY_SCORES = {
    'team_naming_2026': {
      name:         'Team Naming Challenge',
      icon:         'local_activity',
      status:       'completed',
      period_label: 'Apr 8–14, 2026',
      scoring_desc: 'All teams earn a base score for participating. Bonus awarded for full team participation and the most creative name.',
      max_pts:      { base: 100, participation: 20, creative: 25 },
      entries: [
        { team_name: 'Vortex Velocity', base_pts: 100, participation_pts: 0,  creative_pts: 25 },
        { team_name: 'Neon Stride',     base_pts: 100, participation_pts: 0,  creative_pts: 0  },
        { team_name: 'Apex Pulse',      base_pts: 100, participation_pts: 20, creative_pts: 0  },
        { team_name: 'Thunder Pace',    base_pts: 100, participation_pts: 0,  creative_pts: 0  },
        { team_name: 'Solar Sprint',    base_pts: 100, participation_pts: 0,  creative_pts: 0  },
        { team_name: 'Kinetic Edge',    base_pts: 100, participation_pts: 0,  creative_pts: 0  },
      ],
    },
  };

  function resolveDate(label) {
    if (label === 'Today')     return new Date(CY, CM, CD);
    if (label === 'Yesterday') return new Date(CY, CM, CD - 1);
    return new Date(label);
  }

  // ═══════════════════════════════════════════════════════════════════
  // ISO WEEK (Monday-based)
  // ═══════════════════════════════════════════════════════════════════
  function isoWeekOf(date) {
    var d   = new Date(date.getTime());
    var day = d.getDay() || 7;
    d.setDate(d.getDate() + 4 - day);
    var jan1 = new Date(d.getFullYear(), 0, 1);
    return { year: d.getFullYear(), week: Math.ceil(((d - jan1) / 86400000 + 1) / 7) };
  }

  // ═══════════════════════════════════════════════════════════════════
  // TIER LOGIC  (mirrors crates/scoring/src/walk_run.rs)
  // ═══════════════════════════════════════════════════════════════════
  function tierFor(km) {
    if (km >= 25) return { name: 'ELITE',   bonus: 25 };
    if (km >= 20) return { name: 'PACER',   bonus: 20 };
    if (km >= 15) return { name: 'STRIDER', bonus: 15 };
    if (km >= 10) return { name: 'RUNNER',  bonus: 10 };
    if (km >= 5)  return { name: 'MOVER',   bonus: 5  };
    if (km >= 1)  return { name: 'STARTER', bonus: 0  };
    return             { name: 'INACTIVE', bonus: 0  };
  }

  // ═══════════════════════════════════════════════════════════════════
  // TEAM LOOKUP — derived from TEAM_ROSTER above
  // ═══════════════════════════════════════════════════════════════════
  var athleteToTeam = {};
  var teamIdMap     = {};
  var nextId        = 1;

  Object.keys(TEAM_ROSTER).forEach(function (teamName) {
    if (!teamIdMap[teamName]) teamIdMap[teamName] = nextId++;
    TEAM_ROSTER[teamName].forEach(function (name) { athleteToTeam[name] = teamName; });
  });

  function displayName(n) { return n; }

  function teamFor(name) { return athleteToTeam[name] || 'Unassigned'; }
  function idFor(teamName) {
    if (!teamIdMap[teamName]) teamIdMap[teamName] = nextId++;
    return teamIdMap[teamName];
  }

  // ═══════════════════════════════════════════════════════════════════
  // PRE-PROCESS: aggregate km per athlete per week
  // ═══════════════════════════════════════════════════════════════════
  var weeklyMap    = {};   // "athlete|year|week" → { athlete, year, week, totalKm }
  var rawWithDates = [];

  RAW.forEach(function (row) {
    var id = row[0], athlete = row[1], km = row[2], label = row[3];
    var date = resolveDate(label);
    var yw   = isoWeekOf(date);
    rawWithDates.push({ id: id, athlete: athlete, km: km, date: date, year: yw.year, week: yw.week, label: label });
    if (date < GAME_START) return; // ignore activities before competition start (April 8, 2026)
    var key = athlete + '|' + yw.year + '|' + yw.week;
    if (!weeklyMap[key]) weeklyMap[key] = { athlete: athlete, year: yw.year, week: yw.week, totalKm: 0 };
    weeklyMap[key].totalKm += km;
  });

  // ═══════════════════════════════════════════════════════════════════
  // PUBLIC API
  // ═══════════════════════════════════════════════════════════════════

  function getIndividuals(year, week, opts) {
    opts     = opts || {};
    var page     = opts.page     || 1;
    var per_page = opts.per_page || 20;

    var all = [];
    Object.keys(weeklyMap).forEach(function (key) {
      var e = weeklyMap[key];
      if (e.year !== year || e.week !== week) return;
      var t  = tierFor(e.totalKm);
      var km = Math.round(e.totalKm * 100) / 100;
      all.push({
        user_name:    displayName(e.athlete),
        team_name:    teamFor(e.athlete),
        raw_value:    km,
        tier_name:    t.name,
        base_points:  km,
        bonus_points: t.bonus,
        total_points: Math.round((km + t.bonus) * 100) / 100,
      });
    });

    all.sort(function (a, b) { return b.total_points - a.total_points; });
    all.forEach(function (e, i) { e.rank = i + 1; });

    var start = (page - 1) * per_page;
    return Promise.resolve({ game: 'walk_run_2026', year: year, week: week, entries: all.slice(start, start + per_page) });
  }

  function getTeams(year, week) {
    // Collect all teams that have at least one assigned athlete
    var knownTeams = {};
    Object.keys(athleteToTeam).forEach(function (a) { knownTeams[athleteToTeam[a]] = true; });

    // Active athletes (logged km) this week
    var teamStats = {};
    Object.keys(weeklyMap).forEach(function (key) {
      var e    = weeklyMap[key];
      if (e.year !== year || e.week !== week) return;
      var team = teamFor(e.athlete);
      if (!teamStats[team]) teamStats[team] = { kms: [] };
      teamStats[team].kms.push(e.totalKm);
    });

    // Total member count per team from current assignments
    var totalCnt = {};
    Object.keys(athleteToTeam).forEach(function (a) {
      var t = athleteToTeam[a];
      totalCnt[t] = (totalCnt[t] || 0) + 1;
    });

    var teams = [];
    Object.keys(knownTeams).forEach(function (teamName) {
      var stats    = teamStats[teamName] || { kms: [] };
      var active   = stats.kms.length;
      var total    = totalCnt[teamName] || 0;
      var kmSum    = stats.kms.reduce(function(s, v) { return s + v; }, 0);
      var avgDist  = Math.round((total > 0 ? kmSum / total : 0) * 100) / 100;
      var avgBonus = tierFor(avgDist).bonus; // bonus tier applied to team avg distance
      teams.push({
        team_id:             idFor(teamName),
        team_name:           teamName,
        avg_score:           Math.round((avgDist + avgBonus) * 100) / 100,
        avg_distance:        avgDist,
        avg_bonus:           avgBonus,
        active_member_count: active,
        total_member_count:  total,
      });
    });

    teams.sort(function (a, b) { return b.avg_score - a.avg_score; });
    teams.forEach(function (t, i) { t.rank = i + 1; });
    return Promise.resolve({ game: 'walk_run_2026', year: year, week: week, entries: teams });
  }

  function getRecentActivity(n) {
    n = n || 10;
    return rawWithDates
      .slice()
      .sort(function (a, b) { return b.date - a.date; })
      .slice(0, n)
      .map(function (e) {
        return { id: e.id, athlete_name: displayName(e.athlete), distance: e.km,
                 team_name: teamFor(e.athlete), date_label: e.label, date: e.date };
      });
  }

  function getCaptureWeek() {
    return isoWeekOf(new Date(CY, CM, CD));
  }

  // ═══════════════════════════════════════════════════════════════════
  // SEASON STANDINGS — sums all weekly team scores + challenge scores
  // ═══════════════════════════════════════════════════════════════════
  function getSeasonTeams() {
    var weeksSet = {};
    Object.keys(weeklyMap).forEach(function(key) {
      var e = weeklyMap[key];
      weeksSet[e.year + '_' + e.week] = { year: e.year, week: e.week };
    });
    var weekList = Object.keys(weeksSet).map(function(k) { return weeksSet[k]; })
      .sort(function(a, b) { return (a.year * 100 + a.week) - (b.year * 100 + b.week); });

    return Promise.all(weekList.map(function(yw) { return getTeams(yw.year, yw.week); }))
      .then(function(results) {
        var teamSeason = {};
        results.forEach(function(result) {
          result.entries.forEach(function(team) {
            if (!teamSeason[team.team_name]) {
              teamSeason[team.team_name] = {
                team_id:             team.team_id,
                strava_pts:          0,
                active_member_count: team.active_member_count,
                total_member_count:  team.total_member_count,
              };
            }
            var s         = teamSeason[team.team_name];
            s.strava_pts += team.avg_score;
            s.active_member_count = team.active_member_count;
            s.total_member_count  = team.total_member_count;
          });
        });

        // Accumulate challenge scores per team
        var challengeTotals = {};
        Object.keys(ACTIVITY_SCORES).forEach(function(actId) {
          (ACTIVITY_SCORES[actId].entries || []).forEach(function(e) {
            if (!challengeTotals[e.team_name]) challengeTotals[e.team_name] = {};
            challengeTotals[e.team_name][actId] =
              (e.base_pts || 0) + (e.participation_pts || 0) + (e.creative_pts || 0);
          });
        });

        var teams = Object.keys(teamSeason).map(function(teamName) {
          var s         = teamSeason[teamName];
          var acts      = challengeTotals[teamName] || {};
          var actTotal  = Object.keys(acts).reduce(function(sum, k) { return sum + acts[k]; }, 0);
          return {
            team_id:             s.team_id,
            team_name:           teamName,
            strava_pts:          Math.round(s.strava_pts  * 100) / 100,
            naming_pts:          acts['team_naming_2026'] || 0,
            activity_pts:        actTotal,
            total_pts:           Math.round((s.strava_pts + actTotal) * 100) / 100,
            active_member_count: s.active_member_count,
            total_member_count:  s.total_member_count,
          };
        });

        teams.sort(function(a, b) { return b.total_pts - a.total_pts; });
        teams.forEach(function(t, i) { t.rank = i + 1; });
        return { season: 2026, entries: teams };
      });
  }

  function getActivityScores(activityId) {
    var activity = ACTIVITY_SCORES[activityId];
    if (!activity) return Promise.resolve(null);
    var entries = (activity.entries || []).map(function(e) {
      return Object.assign({}, e, {
        total_pts: (e.base_pts || 0) + (e.participation_pts || 0) + (e.creative_pts || 0),
      });
    });
    entries.sort(function(a, b) { return b.total_pts - a.total_pts; });
    entries.forEach(function(e, i) { e.rank = i + 1; });
    return Promise.resolve(Object.assign({}, activity, { id: activityId, entries: entries }));
  }

  function getActivitiesList() {
    return Promise.resolve(Object.keys(ACTIVITY_SCORES).map(function(id) {
      var a = ACTIVITY_SCORES[id];
      return { id: id, name: a.name, icon: a.icon, status: a.status,
               period_label: a.period_label, scoring_desc: a.scoring_desc,
               max_pts: a.max_pts, entry_count: (a.entries || []).length };
    }));
  }

  // ═══════════════════════════════════════════════════════════════════
  // STRAVA SEASON — per-team distance/bonus/pts across all weeks with weekly breakdown
  // ═══════════════════════════════════════════════════════════════════
  function getStravaTeamsSeason() {
    var weeksSet = {};
    Object.keys(weeklyMap).forEach(function(key) {
      var e = weeklyMap[key];
      weeksSet[e.year + '_' + e.week] = { year: e.year, week: e.week };
    });
    var weekList = Object.keys(weeksSet).map(function(k) { return weeksSet[k]; })
      .sort(function(a, b) { return (a.year * 100 + a.week) - (b.year * 100 + b.week); });

    return Promise.all(weekList.map(function(yw) { return getTeams(yw.year, yw.week); }))
      .then(function(results) {
        var teamData = {};
        results.forEach(function(result, wi) {
          var yw = weekList[wi];
          result.entries.forEach(function(team) {
            if (!teamData[team.team_name]) {
              teamData[team.team_name] = {
                team_id:             team.team_id,
                team_name:           team.team_name,
                total_distance:      0,
                total_bonus:         0,
                total_pts:           0,
                active_member_count: team.active_member_count,
                total_member_count:  team.total_member_count,
                weeks:               {},
              };
            }
            var td = teamData[team.team_name];
            td.total_distance      += team.avg_distance;
            td.total_bonus         += team.avg_bonus;
            td.total_pts           += team.avg_score;
            td.active_member_count  = team.active_member_count;
            td.total_member_count   = team.total_member_count;
            td.weeks[yw.year + '_' + yw.week] = {
              year: yw.year, week: yw.week,
              avg_distance:        team.avg_distance,
              avg_bonus:           team.avg_bonus,
              avg_score:           team.avg_score,
              active_member_count: team.active_member_count,
            };
          });
        });

        var entries = Object.keys(teamData).map(function(name) {
          var td = teamData[name];
          return {
            team_id:             td.team_id,
            team_name:           td.team_name,
            total_distance:      Math.round(td.total_distance * 100) / 100,
            total_bonus:         Math.round(td.total_bonus    * 100) / 100,
            total_pts:           Math.round(td.total_pts      * 100) / 100,
            active_member_count: td.active_member_count,
            total_member_count:  td.total_member_count,
            weeks:               td.weeks,
          };
        });

        entries.sort(function(a, b) { return b.total_pts - a.total_pts; });
        entries.forEach(function(e, i) { e.rank = i + 1; });
        return {
          weeks: weekList.map(function(yw) {
            return { year: yw.year, week: yw.week, key: yw.year + '_' + yw.week };
          }),
          entries: entries,
        };
      });
  }

  global.CubeData = {
    getIndividuals:    getIndividuals,
    getTeams:          getTeams,
    getRecentActivity: getRecentActivity,
    getCaptureWeek:    getCaptureWeek,
    getSeasonTeams:       getSeasonTeams,
    getStravaTeamsSeason: getStravaTeamsSeason,
    getActivityScores: getActivityScores,
    getActivitiesList: getActivitiesList,
  };

})(window);
