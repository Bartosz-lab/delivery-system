# Delivery System

The aim of this application is to streamline the parcel management process.

## Main Features

* ### Trade Partner Functions
    * #### Parcel Registration
        The application allows for the registration of new parcels in the system. During registration, the customer provides recipient details and parcel information, such as size and weight. They also select a predefined sending warehouse and specify the date of parcel pickup.
    * #### Parcel Tracking
        The application enables tracking of the parcel's status.
    * #### Settlement Reports
        The application allows for generating reports regarding the costs incurred by the customer in relation to the dispatched parcels. Daily, weekly, and monthly reports are available, with the option to filter by sending warehouses.

* ### Courier Company Employee Functions
    * #### Registration of New Parcel Status
        The application enables registration of a new status and location for a parcel. This facilitates tracking the route taken by the package and automatically sending relevant notifications to the customer and/or recipient associated with specific statuses.
    * #### Reports
        The application allows for generating the following reports:
        * Information about parcels to be collected from specific sending warehouses.
        * Information about parcels received for delivery.
        * Information about customer settlements.

* ### Recipient Functions
    * #### Parcel Tracking
        The application enables tracking of the parcel's status.
    * #### Parcel Management
        The application allows for changing the delivery date of a package and redirecting it to a different address.


## Business Logic

* Parcel Lifecycle:
    1. Parcel Registration:

        The starting point of the parcel's lifecycle, initiated by the trade partner.
        After submitting a request to create a parcel in the system, a unique waybill number is assigned to the parcel.
    2. Sending an email/SMS to the recipient with access to the parcel management panel.

    3. Adding new parcel statuses by courier company employees:
        * After parcel collection - sending a notification to the recipient
        * After reaching transfer points
        * After being released for delivery - sending a notification to the recipient
    
    4. Parcel Delivery:
        Sending a notification to the recipient and marking the parcel as delivered.

    The recipient can modify the delivery date and address if the parcel has a status that allows for such changes.

* Trade Partner:
    Separate rates are defined for specific types of parcels (weight, size) for each customer.
    Trade Partner can request settlement reports, and the requested data will be calculated accordingly.


## List of Key Business Entities:
* ### Parcel:
    * Properties:
        * Recipient Address
        * Recipient Name/Organization
        * Recipient Email
        * Recipient Phone
        * Sending Warehouse
        * Pickup Date
    * Functionality
        * Status Change
        * Pickup Location Change
        * Pickup Date Change

* ### Trade Parnter:
    * Properties:
        * Name
        * Price List
        * List of Sending Warehouses
    * Functionality
        * Management of Sending Warehouses

## Services

* ### Reporting
    * #### Settlements
        A service used for generating reports for trading partners with information about settlements.

    * #### Parcels
        A service used for generating reports for couriers with information about parcels.
