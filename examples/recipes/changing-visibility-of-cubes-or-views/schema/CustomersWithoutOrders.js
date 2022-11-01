view('CustomersWithoutOrders', {
  description: `Customers without orders`,

  includes: [
    Users.company,
  ],

  measures: {
    count: {
      // Note the nested path to `Orders.count`
      sql: `${Users.Orders.count}`,
      // Here we explicitly set the type as `number`, as we don't want to re-calculate
      // the count
      type: `number`,
      // We use the nested path to `Orders.count` again, but this time to filter
      // results so that we only get users with no orders
      filters: [{ sql: `${Users.Orders.count} = 0` }],
    },
  },
});
