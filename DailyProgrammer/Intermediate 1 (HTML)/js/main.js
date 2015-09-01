$("#addEventButton").click(function()
{
    // Show the modal dialog.
    $("#addEventModal").modal("show");
});

$("#addEventConfirmButton").click(function()
{
    // Add a new event entry.
    var element = $("<tr>");
    element.append($("<td>").text($("#inputDate").val()));
    element.append($("<td>").text($("#inputTime").val()));
    element.append($("<td>").text($("#inputDescription").val()));
    element.append($("<td>").append($("<span>").addClass("glyphicon glyphicon-edit")));

    $("#eventList").append(element);

    // Hide the modal dialog.
    $("#addEventModal").modal("hide");

    // Clear the dialog form.
    $("#addEventForm")[0].reset();
});
