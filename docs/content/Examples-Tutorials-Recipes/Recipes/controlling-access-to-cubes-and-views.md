---
title: Controlling access to cubes and views
permalink: /recipes/controlling-access-to-cubes-and-views
category: Examples & Tutorials
subCategory: Access control
menuOrder: 1
---

## Use case

We want to manage user access to different cubes and/or views depending on some
sort of user property. In the recipe below, we will manage access to a view so
that only users with a `department` claim in their JWT can query it.

## Configuration

```javascript
module.exports = {
  contextToAppId: ({ securityContext }) => {
    return `CUBEJS_APP_${securityContext.company}`;
  },
  extendContext: (req) => {
    const { department } = jwtDecode(req.headers['authorization']);
    return {
      permissions: {
        finance: department === 'finance',
      },
    };
  },
};
```

## Data schema

```javascript
// Orders.js
cube(`Orders`, {
  sql: `SELECT * FROM public.orders`,
  shown: false,

  ...,
});

// Users.js
cube(`Users`, {
  sql: `SELECT * FROM public.users`,
  shown: false,

  ...,
});

// TotalRevenuePerCustomer.js
view('TotalRevenuePerCustomer', {
	description: `Total revenue per customer`,
  shown: COMPILE_CONTEXT.permissions['finance'],

	includes: [
		Orders.totalRevenue,
		Users.company,
	],
});
```

## Query

After generating a JWT with a `department` claim set to `finance`, we can send
it as part of a cURL command:

```bash
curl \
  -H "Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJkZXBhcnRtZW50IjoiZmluYW5jZSIsImV4cCI6MTY2NzMzNzI1MH0.njfL7GMDNlzKaJDZA0OQ_b2u2JhuSm-WjnS0yVfB8NA" \
  http://localhost:4000/cubejs-api/v1/meta
```

## Result

The `/meta` endpoint shows the available cubes and views:

```json
{
  "cubes": [
    {
      "name": "CustomersWithoutOrders",
      "title": "Customers Without Orders",
      "description": "Customers without orders",
      "measures": [
        {
          "name": "CustomersWithoutOrders.count",
          "title": "Customers Without Orders Count",
          "shortTitle": "Count",
          "cumulativeTotal": false,
          "cumulative": false,
          "type": "number",
          "aggType": "number",
          "drillMembers": [],
          "drillMembersGrouped": {
            "measures": [],
            "dimensions": []
          },
          "isVisible": true
        }
      ],
      "dimensions": [
        {
          "name": "CustomersWithoutOrders.company",
          "title": "Customers Without Orders Company",
          "type": "string",
          "shortTitle": "Company",
          "suggestFilterValues": true,
          "isVisible": true
        }
      ],
      "segments": []
    },
    {
      "name": "Orders",
      "title": "Orders",
      "connectedComponent": 1,
      "measures": [
        {
          "name": "Orders.count",
          "title": "Orders Count",
          "shortTitle": "Count",
          "cumulativeTotal": false,
          "cumulative": false,
          "type": "number",
          "aggType": "count",
          "drillMembers": ["Orders.id", "Orders.createdAt"],
          "drillMembersGrouped": {
            "measures": [],
            "dimensions": ["Orders.id", "Orders.createdAt"]
          },
          "isVisible": false
        },
        {
          "name": "Orders.totalRevenue",
          "title": "Orders Total Revenue",
          "shortTitle": "Total Revenue",
          "cumulativeTotal": false,
          "cumulative": false,
          "type": "number",
          "aggType": "sum",
          "drillMembers": [],
          "drillMembersGrouped": {
            "measures": [],
            "dimensions": []
          },
          "isVisible": false
        }
      ],
      "dimensions": [
        {
          "name": "Orders.id",
          "title": "Orders Id",
          "type": "number",
          "shortTitle": "Id",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Orders.status",
          "title": "Orders Status",
          "type": "string",
          "shortTitle": "Status",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Orders.createdAt",
          "title": "Orders Created at",
          "type": "time",
          "shortTitle": "Created at",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Orders.completedAt",
          "title": "Orders Completed at",
          "type": "time",
          "shortTitle": "Completed at",
          "suggestFilterValues": true,
          "isVisible": false
        }
      ],
      "segments": []
    },
    {
      "name": "TotalRevenuePerCustomer",
      "title": "Total Revenue Per Customer",
      "description": "Total revenue per customer",
      "measures": [
        {
          "name": "TotalRevenuePerCustomer.totalRevenue",
          "title": "Total Revenue Per Customer Total Revenue",
          "shortTitle": "Total Revenue",
          "cumulativeTotal": false,
          "cumulative": false,
          "type": "number",
          "aggType": "number",
          "drillMembers": [],
          "drillMembersGrouped": {
            "measures": [],
            "dimensions": []
          },
          "isVisible": false
        }
      ],
      "dimensions": [
        {
          "name": "TotalRevenuePerCustomer.company",
          "title": "Total Revenue Per Customer Company",
          "type": "string",
          "shortTitle": "Company",
          "suggestFilterValues": true,
          "isVisible": false
        }
      ],
      "segments": []
    },
    {
      "name": "Users",
      "title": "Users",
      "connectedComponent": 1,
      "measures": [
        {
          "name": "Users.count",
          "title": "Users Count",
          "shortTitle": "Count",
          "cumulativeTotal": false,
          "cumulative": false,
          "type": "number",
          "aggType": "count",
          "drillMembers": [
            "Users.id",
            "Users.city",
            "Users.firstName",
            "Users.lastName",
            "Users.createdAt"
          ],
          "drillMembersGrouped": {
            "measures": [],
            "dimensions": [
              "Users.id",
              "Users.city",
              "Users.firstName",
              "Users.lastName",
              "Users.createdAt"
            ]
          },
          "isVisible": false
        }
      ],
      "dimensions": [
        {
          "name": "Users.id",
          "title": "Users Id",
          "type": "number",
          "shortTitle": "Id",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Users.city",
          "title": "Users City",
          "type": "string",
          "shortTitle": "City",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Users.company",
          "title": "Users Company",
          "type": "string",
          "shortTitle": "Company",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Users.gender",
          "title": "Users Gender",
          "type": "string",
          "shortTitle": "Gender",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Users.firstName",
          "title": "Users First Name",
          "type": "string",
          "shortTitle": "First Name",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Users.lastName",
          "title": "Users Last Name",
          "type": "string",
          "shortTitle": "Last Name",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Users.state",
          "title": "Users State",
          "type": "string",
          "shortTitle": "State",
          "suggestFilterValues": true,
          "isVisible": false
        },
        {
          "name": "Users.createdAt",
          "title": "Users Created at",
          "type": "time",
          "shortTitle": "Created at",
          "suggestFilterValues": true,
          "isVisible": false
        }
      ],
      "segments": []
    }
  ]
}
```

## Source code

Please feel free to check out the
[full source code](https://github.com/cube-js/cube.js/tree/master/examples/recipes/changing-visibility-of-cubes-or-views)
or run it with the `docker-compose up` command.
