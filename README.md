# soccer-sentiment-API-saas
An example on building an mvp saas in a day. Testing out deploying an API as a service. This is part of a another project. It's only visible because I had to make it so in order to deploy on github pages. 

This is not a serious app, but it works and I'm happy

https://viukpe.github.io/soccer-sentiment-API-saas/

## API Endpoints
Soccer Sentiment API — Endpoint Reference
Base URL
https://api.soccer-sentiment.com/v1

1. GET /sentiments
Returns all team sentiment records. Supports filtering, sorting, and topic search via query parameters.
Query Parameters
ParameterTypeDescriptionExampleleaguestringFilter by league IDpremier-leagueseasonstringFilter by season2025/26labelstringFilter by sentiment labelPositiveteamstringFilter by team name (partial match)ArsenaltopicstringFilter by keyword in key topicsChampions LeaguesortstringField to sort byoverall_scoreorderstringSort direction: asc or descdesc
Example Request
GET /sentiments?league=premier-league&sort=overall_score&order=desc
Example Response
json{
  "count": 4,
  "filters_applied": {
    "league": "premier-league",
    "sort": "overall_score",
    "order": "desc"
  },
  "data": [
    {
      "id": 2,
      "team_id": "manchester-city",
      "team_name": "Manchester City FC",
      "league_id": "premier-league",
      "league_name": "Premier League",
      "season": "2025/26",
      "snapshot_date": "2026-04-15",
      "overall_score": 81.0,
      "overall_label": "Very Positive",
      "scores": {
        "performance": 85.0,
        "management": 82.0,
        "transfers": 79.0,
        "atmosphere": 77.0
      },
      "key_topics": ["Haaland", "squad depth", "Guardiola", "Champions League"],
      "fan_voice_summary": "City fans are in high spirits as the team continues to dominate domestically and look threatening in Europe. Guardiola's system is clicking and Haaland is in lethal form. Confidence in the squad is at a season high.",
      "positive_highlights": [
        "Haaland's goal tally",
        "Guardiola's rotation management",
        "Defensive solidity"
      ],
      "negative_highlights": [
        "Fixture congestion concerns",
        "Fatigue in the full-back positions"
      ]
    },
    {
      "id": 3,
      "team_id": "liverpool",
      "team_name": "Liverpool FC",
      "league_id": "premier-league",
      "league_name": "Premier League",
      "season": "2025/26",
      "snapshot_date": "2026-04-15",
      "overall_score": 76.0,
      "overall_label": "Positive",
      "scores": {
        "performance": 80.0,
        "management": 74.0,
        "transfers": 68.0,
        "atmosphere": 85.0
      },
      "key_topics": ["Slot tactics", "Salah contract", "Anfield atmosphere", "top four"],
      "fan_voice_summary": "Liverpool fans are enjoying a resurgent season under Slot, though the unresolved Salah contract situation remains a source of anxiety. The atmosphere at Anfield has been outstanding and the front line is firing on all cylinders.",
      "positive_highlights": [
        "Anfield atmosphere at its peak",
        "Front three clicking",
        "Solid defensive shape"
      ],
      "negative_highlights": [
        "Salah contract uncertainty",
        "Midfield cover thin"
      ]
    },
    {
      "id": 1,
      "team_id": "arsenal",
      "team_name": "Arsenal FC",
      "league_id": "premier-league",
      "league_name": "Premier League",
      "season": "2025/26",
      "snapshot_date": "2026-04-15",
      "overall_score": 72.5,
      "overall_label": "Positive",
      "scores": {
        "performance": 78.0,
        "management": 70.0,
        "transfers": 65.0,
        "atmosphere": 80.0
      },
      "key_topics": ["title race", "Saka", "Arteta tactics", "Emirates atmosphere"],
      "fan_voice_summary": "Fans are broadly optimistic about Arsenal's title chances this season, with Saka's form drawing particular praise. Some concern remains over squad depth, but the mood at the Emirates is electric.",
      "positive_highlights": [
        "Saka in world-class form",
        "Arteta's tactical flexibility",
        "Strong home record"
      ],
      "negative_highlights": [
        "Injury concerns in midfield",
        "Inconsistency away from home"
      ]
    },
    {
      "id": 4,
      "team_id": "chelsea",
      "team_name": "Chelsea FC",
      "league_id": "premier-league",
      "league_name": "Premier League",
      "season": "2025/26",
      "snapshot_date": "2026-04-15",
      "overall_score": 31.0,
      "overall_label": "Negative",
      "scores": {
        "performance": 35.0,
        "management": 28.0,
        "transfers": 40.0,
        "atmosphere": 38.0
      },
      "key_topics": ["ownership", "manager instability", "transfer spend", "inconsistency"],
      "fan_voice_summary": "Chelsea fans are frustrated and disillusioned after another chaotic season. Despite enormous transfer outlay the squad lacks cohesion and the manager appears to have lost the dressing room. Calls for a clear sporting vision are growing louder.",
      "positive_highlights": [
        "Individual moments of brilliance from young signings",
        "Stamford Bridge still sells out"
      ],
      "negative_highlights": [
        "No clear playing identity",
        "Manager under severe pressure",
        "Wasted transfer budget"
      ]
    }
  ]
}

2. GET /sentiments (filtered by topic)
Example Request
GET /sentiments?topic=Champions League
Example Response
json{
  "count": 3,
  "filters_applied": {
    "topic": "Champions League"
  },
  "data": [
    {
      "id": 5,
      "team_id": "real-madrid",
      "team_name": "Real Madrid CF",
      "league_id": "la-liga",
      "league_name": "La Liga",
      "season": "2025/26",
      "snapshot_date": "2026-04-15",
      "overall_score": 88.5,
      "overall_label": "Very Positive",
      "scores": {
        "performance": 90.0,
        "management": 87.0,
        "transfers": 85.0,
        "atmosphere": 91.0
      },
      "key_topics": ["Vinicius Jr", "Bellingham", "Champions League", "Ancelotti"],
      "fan_voice_summary": "Real Madrid fans are euphoric as the team storms through La Liga and dominates Europe once again. Vinicius and Bellingham are in scintillating form and Ancelotti's calm authority is widely praised. The Bernabeu is rocking.",
      "positive_highlights": [
        "Vinicius in Ballon d'Or form",
        "Bellingham's leadership",
        "Champions League pedigree showing"
      ],
      "negative_highlights": [
        "Defensive injuries a worry",
        "Reliance on two players"
      ]
    },
    {
      "id": 2,
      "team_id": "manchester-city",
      "team_name": "Manchester City FC",
      "league_id": "premier-league",
      "league_name": "Premier League",
      "season": "2025/26",
      "snapshot_date": "2026-04-15",
      "overall_score": 81.0,
      "overall_label": "Very Positive",
      "scores": {
        "performance": 85.0,
        "management": 82.0,
        "transfers": 79.0,
        "atmosphere": 77.0
      },
      "key_topics": ["Haaland", "squad depth", "Guardiola", "Champions League"],
      "fan_voice_summary": "City fans are in high spirits as the team continues to dominate domestically and look threatening in Europe.",
      "positive_highlights": [
        "Haaland's goal tally",
        "Guardiola's rotation management",
        "Defensive solidity"
      ],
      "negative_highlights": [
        "Fixture congestion concerns",
        "Fatigue in the full-back positions"
      ]
    },
    {
      "id": 8,
      "team_id": "psg",
      "team_name": "Paris Saint-Germain",
      "league_id": "ligue-1",
      "league_name": "Ligue 1",
      "season": "2025/26",
      "snapshot_date": "2026-04-15",
      "overall_score": 47.5,
      "overall_label": "Mixed",
      "scores": {
        "performance": 55.0,
        "management": 44.0,
        "transfers": 42.0,
        "atmosphere": 50.0
      },
      "key_topics": ["post-Mbappe era", "sporting project", "Champions League", "ownership expectations"],
      "fan_voice_summary": "PSG fans are skeptical after yet another underwhelming Champions League campaign despite massive investment.",
      "positive_highlights": [
        "Dominant in Ligue 1",
        "Some exciting young talent emerging"
      ],
      "negative_highlights": [
        "Champions League ceiling still not broken",
        "Lack of identity post-Mbappe",
        "Fan engagement at domestic games low"
      ]
    }
  ]
}

3. GET /leagues
Returns a list of all distinct leagues available in the dataset.
Example Request
GET /leagues
Example Response
json{
  "count": 4,
  "data": [
    {
      "league_id": "premier-league",
      "league_name": "Premier League",
      "team_count": 4
    },
    {
      "league_id": "la-liga",
      "league_name": "La Liga",
      "team_count": 2
    },
    {
      "league_id": "bundesliga",
      "league_name": "Bundesliga",
      "team_count": 1
    },
    {
      "league_id": "ligue-1",
      "league_name": "Ligue 1",
      "team_count": 1
    }
  ]
}

4. GET /seasons
Returns a list of all distinct seasons available in the dataset.
Example Request
GET /seasons
Example Response
json{
  "count": 1,
  "data": [
    {
      "season": "2025/26",
      "snapshot_count": 8,
      "latest_snapshot_date": "2026-04-15"
    }
  ]
}

HTTP Status Codes
CodeMeaning200Success400Bad request (invalid query parameter)404No data found matching the applied filters500Internal server error
Error Response Format
json{
  "error": {
    "code": 400,
    "message": "Invalid value for 'sort'. Allowed values: overall_score, performance_score, management_score, transfers_score, atmosphere_score."
  }
}
